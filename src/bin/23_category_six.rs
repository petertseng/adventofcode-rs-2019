enum TxFunc {
    AddRxs,
    MultiplyRxs,
    DivideRxs,
    FirstRx,
}

struct Nic {
    sent: bool,
    slot_divisor: usize,
    rxs: Vec<Option<i64>>,
    tx_f: TxFunc,
    txs: Vec<(u8, usize)>,
}

type Packet = (u8, usize, i64);

impl Nic {
    fn new(comp: &adventofcode::intcode::Computer) -> Self {
        let to_b = |i: i64| match i {
            1 => true,
            0 => false,
            n => panic!("bad bool {}", n),
        };

        let rx_slots = usize::try_from(comp.get(67)).expect("bad rx slots");
        let rx_base = usize::try_from(comp.get(68)).expect("bad rx base");
        let rxs = (0..rx_slots).map(|i| {
            let present = to_b(comp.get(rx_base + i * 2));
            if present {
                Some(comp.get(rx_base + i * 2 + 1))
            } else {
                None
            }
        });
        let rxs = rxs.collect();

        let num_txs = usize::try_from(comp.get(71)).expect("bad num txs");
        let tx_base = usize::try_from(comp.get(72)).expect("bad tx base");
        let txs = (0..num_txs).map(|i| {
            let id = u8::try_from(comp.get(tx_base + 2 * i)).expect("bad tx id");
            let x = usize::try_from(comp.get(tx_base + 2 * i + 1)).expect("bad tx x");
            (id, x)
        });
        let txs = txs.collect();

        let tx_f = match comp.get(69) {
            253 => TxFunc::AddRxs,
            302 => TxFunc::MultiplyRxs,
            351 => TxFunc::DivideRxs,
            556 => TxFunc::FirstRx,
            n => panic!("bad tx_f {}", n),
        };

        Self {
            sent: to_b(comp.get(61)),
            slot_divisor: usize::try_from(comp.get(66)).expect("bad slot divisor"),
            rxs,
            tx_f,
            txs,
        }
    }

    fn y(&self, mut rxs: impl Iterator<Item = i64>) -> i64 {
        match self.tx_f {
            TxFunc::AddRxs => rxs.sum(),
            TxFunc::MultiplyRxs => rxs.product(),
            TxFunc::DivideRxs => {
                let a = rxs.next().unwrap();
                let b = rxs.next().unwrap();
                a / b
            }
            TxFunc::FirstRx => rxs.next().unwrap(),
        }
    }

    fn no_packet(&mut self) -> Vec<Packet> {
        if self.sent {
            Vec::new()
        } else {
            self.send_packets()
        }
    }

    fn receive_packet(&mut self, x: usize, y: i64) -> Vec<Packet> {
        let rx_slot = x / self.slot_divisor;
        if rx_slot == 0 || rx_slot > self.rxs.len() || self.rxs[rx_slot - 1] == Some(y) {
            return Vec::new();
        }
        self.rxs[rx_slot - 1] = Some(y);
        self.send_packets()
    }

    fn send_packets(&mut self) -> Vec<Packet> {
        self.sent = true;
        if self.rxs.iter().any(|&rx| rx == None) {
            return Vec::new();
        }
        let rxs = self.rxs.iter().map(|&rx| rx.unwrap());
        let y = self.y(rxs);
        self.txs.iter().map(|&(addr, x)| (addr, x, y)).collect()
    }
}

fn run_nics(nics: &mut [Nic]) -> (i64, i64) {
    use std::collections::VecDeque;
    let mut qs: Vec<VecDeque<_>> = vec![VecDeque::new(); nics.len()];

    let mut nat = None;
    let mut first_y_to_nat = None;
    let mut last_y_from_nat = None;

    loop {
        let sent = nics.iter_mut().zip(qs.iter_mut()).flat_map(|(nic, q)| {
            if q.is_empty() {
                nic.no_packet()
            } else {
                q.drain(..)
                    .flat_map(|(x, y)| nic.receive_packet(x, y))
                    .collect()
            }
        });
        let sent: Vec<_> = sent.collect();

        for (addr, x, y) in sent {
            if addr == 255 {
                first_y_to_nat = first_y_to_nat.or(Some(y));
                nat = Some((x, y));
            } else {
                qs[usize::from(addr)].push_back((x, y));
            }
        }

        if qs.iter().all(VecDeque::is_empty) {
            if let Some(pkt) = nat {
                let (_, y) = pkt;
                if last_y_from_nat == Some(y) {
                    return (first_y_to_nat.unwrap(), y);
                }
                last_y_from_nat = Some(y);
                qs[0].push_back(pkt);
            } else {
                panic!("empty queues without nat");
            }
        }
    }
}

fn nics(mem: &[i64], n: u8) -> Vec<Nic> {
    let nics = (0..n).map(|i| {
        let mut ic = adventofcode::intcode::Computer::new(mem);
        ic.cont_in(i64::from(i));
        Nic::new(&ic)
    });
    nics.collect()
}

fn main() {
    let mem = adventofcode::read_input_file_or_intcode();

    let mut nics = nics(&mem, 50);
    let (a, b) = run_nics(&mut nics);
    println!("{}", a);
    println!("{}", b);
}

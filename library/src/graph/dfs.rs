/// DFS の抽象化ライブラリ
///
/// `DfsHandler` トレイトを実装した構造体を作り、`dfs()` に渡して利用する
/// 遷移・訪問済みチェックの方法を外部から注入できる。

pub trait DfsHandler {
    type State: Clone;

    /// 現在の状態から遷移可能な状態の一覧を返却
    fn neighbors(&mut self, state: &Self::State) -> Vec<Self::State>;

    /// 現在の頂点が訪問済みかを返却
    fn is_visited(&self, state: &Self::State) -> bool;

    /// 訪問済みにマーク
    fn mark_visited(&mut self, state: &Self::State);

    /// 行きがけ処理。枝刈りのため、falseを返すと子の探索をスキップする。
    fn on_enter(&mut self, _state: &Self::State) -> bool {
        true
    }

    /// 帰りがけ処理
    fn on_leave(&mut self, _state: &Self::State) {}

    /// true を返すと探索全体を打ち切る
    fn should_stop(&self) -> bool {
        false
    }
}

enum Event<S> {
    Enter(S),
    Leave(S),
}

pub fn dfs<H: DfsHandler>(handler: &mut H, starts: impl IntoIterator<Item = H::State>) {
    let mut stack = {
        let mut v: Vec<_> = starts.into_iter().map(Event::Enter).collect();
        v.reverse();
        v
    };

    while let Some(event) = stack.pop() {
        if handler.should_stop() {
            break;
        }
        match event {
            Event::Enter(state) => {
                if handler.is_visited(&state) {
                    continue;
                }
                handler.mark_visited(&state);
                if !handler.on_enter(&state) {
                    handler.on_leave(&state);
                    continue;
                }

                let neighbors = handler.neighbors(&state);
                stack.push(Event::Leave(state));
                for nxt in neighbors.into_iter().rev() {
                    stack.push(Event::Enter(nxt));
                }
            }
            Event::Leave(state) => {
                handler.on_leave(&state);
            }
        }
    }
}

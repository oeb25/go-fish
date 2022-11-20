import { useEffect, useRef, useState } from "react";
import "./index.css";
import * as Card from "./Card";
import * as fish from "fish-wasm";
import { motion, LayoutGroup, AnimatePresence } from "framer-motion";

type GameState = {
  pool: Card[];
  players: PlayerState[];
  announcements: Announcement[];
  stage: GameStage;
};

type GameStage =
  | { name: "Dealing"; who_next: number }
  | { name: "Playing"; who_next: number };

type PlayerState = {
  hand: Card[];
  pairs: Card[][];
};

type PlayerId = number;

type Announcement = {
  player_asking: PlayerId;
  player_asked: PlayerId;
  asked_for: Card[];
  response: Response;
};

type Response =
  | {
      type: "GoFish";
    }
  | { type: "TakeThese"; count: number };

const CARDS =
  "🂡 🂢 🂣 🂤 🂥 🂦 🂧 🂨 🂩 🂪 🂫 🂬 🂭 🂮 🂱 🂲 🂳 🂴 🂵 🂶 🂷 🂸 🂹 🂺 🂻 🂼 🂽 🂾 🃁 🃂 🃃 🃄 🃅 🃆 🃇 🃈 🃉 🃊 🃋 🃌 🃍 🃎 🃑 🃒 🃓 🃔 🃕 🃖 🃗 🃘 🃙 🃚 🃛 🃜 🃝 🃞".split(
    " "
  );

type Card = number;

const Hand = ({ hidden, cards }: { hidden?: boolean; cards: Card[] }) => {
  return (
    <motion.div
      className="flex -space-x-8 select-none group relative self-center"
      layout
    >
      <LayoutGroup id="hand">
        <AnimatePresence>
          {cards
            .slice()
            .sort((a, b) => ((a % 13) - (b % 13)) * 100 + (a - b))
            .map((c) => (
              <motion.div
                key={c}
                layout
                className="transform"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 20 }}
                //  layoutId={`card-${c}`}
              >
                <div
                  key={c}
                  className="group-hover:opacity-80 transition group-hover:hover:opacity-100 hover:z-10 relative hover:[&>*]:-translate-y-2 shadow"
                >
                  <img
                    className="w-24 transition"
                    src={hidden ? Card.BACKS[0] : Object.values(Card.CARDS)[c]}
                  />
                </div>
              </motion.div>
            ))}
        </AnimatePresence>
      </LayoutGroup>
    </motion.div>
  );
};

const getGameState = (engine: fish.Engine): GameState =>
  JSON.parse(engine.game_state());

function App() {
  const [engine, setEngine] = useState(() => fish.Engine.new());
  const game = getGameState(engine);
  const [_, update] = useState(0);

  useEffect(() => {
    const int = setInterval(
      () => {
        engine.step();
        update((u) => u + 1);
      },
      game.stage.name == "Dealing" ? 100 : 1000
    );

    return () => window.clearInterval(int);
  }, [game.stage.name]);

  return (
    <div className="flex flex-col p-10 space-y-10 items-center">
      <div>
        <Hand cards={game.pool} hidden />
      </div>
      <div className="flex justify-evenly space-x-10">
        {game.players.map((p, pid) => (
          <div key={pid}>
            <motion.div
              layout
              className="text-center text-xl font-medium whitespace-nowrap"
            >
              Player {pid + 1} ({p.pairs.length / 4})
            </motion.div>
            <div>
              <Hand cards={p.hand} hidden={pid != 0 && false} />
            </div>
            {pid == game.stage.who_next && (
              <motion.div
                layoutId="turn"
                className="text-green-400 text-center text-xs uppercase font-bold"
              >
                Turn
              </motion.div>
            )}
          </div>
        ))}
      </div>
      <div className="flex flex-col -space-y-10 relative items-center justify-center">
        {game.announcements
          .slice()
          .reverse()
          .slice(0, 10)
          .map((a, i, xs) => (
            <div
              key={game.announcements.length - i}
              className="relative shadow-xl"
              style={{ zIndex: game.announcements.length - i }}
            >
              <Hand cards={a.asked_for} />
            </div>
          ))}
      </div>

      {/* <div className="p-4 flex">
        <Hand cards={[1, 4, 20, 18]} />
      </div>
      <div className="text-[6em] leading-none">
        <div className="flex text-gray-700">
          {CARDS.slice(14 * 0)
            .slice(0, 14)
            .map((c) => (
              <div className="hover:-translate-y-2 hover:scale-110 transition translate-y-0 cursor-move">
                {c}
              </div>
            ))}
        </div>
        <div className="flex text-red-400">
          {CARDS.slice(14 * 1)
            .slice(0, 14)
            .map((c) => (
              <div className="hover:-translate-y-2 hover:scale-110 transition translate-y-0 cursor-move">
                {c}
              </div>
            ))}
        </div>
        <div className="flex text-red-400">
          {CARDS.slice(14 * 2)
            .slice(0, 14)
            .map((c) => (
              <div className="hover:-translate-y-2 hover:scale-110 transition translate-y-0 cursor-move">
                {c}
              </div>
            ))}
        </div>
        <div className="flex text-gray-700">
          {CARDS.slice(14 * 3)
            .slice(0, 14)
            .map((c) => (
              <div className="hover:-translate-y-2 hover:scale-110 transition translate-y-0 cursor-move">
                {c}
              </div>
            ))}
        </div>
      </div> */}
    </div>
  );
}

export default App;

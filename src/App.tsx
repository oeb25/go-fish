import React, { useEffect, useState } from "react";
import "./index.css";
import * as card from "./Card";
import * as fish from "fish-wasm";
import { motion, LayoutGroup, AnimatePresence } from "framer-motion";
import { Game } from "./types";

const CARDS =
  "🂡 🂢 🂣 🂤 🂥 🂦 🂧 🂨 🂩 🂪 🂫 🂬 🂭 🂮 🂱 🂲 🂳 🂴 🂵 🂶 🂷 🂸 🂹 🂺 🂻 🂼 🂽 🂾 🃁 🃂 🃃 🃄 🃅 🃆 🃇 🃈 🃉 🃊 🃋 🃌 🃍 🃎 🃑 🃒 🃓 🃔 🃕 🃖 🃗 🃘 🃙 🃚 🃛 🃜 🃝 🃞".split(
    " "
  );

const inAllSuits = (rank: Rank): Card[] => {
  const rankNumbers = {
    RA: 0,
    R2: 1,
    R3: 2,
    R4: 3,
    R5: 4,
    R6: 5,
    R7: 6,
    R8: 7,
    R9: 8,
    R10: 9,
    RJ: 10,
    RQ: 11,
    RK: 12,
  };

  const r: number = rankNumbers[rank];

  return [r + 0, r + 13, r + 13 * 2, r + 13 * 3] as Card[];
};

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
                    src={hidden ? card.BACKS[0] : Object.values(card.CARDS)[c]}
                  />
                </div>
              </motion.div>
            ))}
        </AnimatePresence>
      </LayoutGroup>
    </motion.div>
  );
};

const getGameState = (engine: fish.Engine): Game =>
  JSON.parse(engine.game_state());

function App() {
  const [engine, setEngine] = useState(() => fish.Engine.new());
  const game = getGameState(engine);
  const [_, update] = useState(0);

  const ROUNDS_PER_TICK = 1;

  useEffect(() => {
    const int = setInterval(
      () => {
        for (let i = 0; i < ROUNDS_PER_TICK; i++) {
          engine.step();
        }
        update((u) => u + 1);
      },
      game.stage.name == "Dealing" ? 100 : 100
    );

    return () => window.clearInterval(int);
  }, [game.stage.name]);

  return (
    <div
      className={
        "grid w-full " +
        (game.stage.name == "Done"
          ? "grid-cols-[40rem_1fr]"
          : "grid-cols-[20rem_1fr_20rem]")
      }
    >
      <div className="relative overflow-y-auto h-screen">
        <div className="bg-slate-900/80 sticky top-0 z-50 p-3 text-xl">
          <span className="text-slate-500">Game:</span>{" "}
          <span className="font-semibold">{game.stage.name}</span>
        </div>
        <div className="relative items-center justify-center grid grid-cols-[auto_1fr_auto] place-items-center gap-2 p-5">
          {game.announcements
            .slice()
            .reverse()
            // .slice(0, 10)
            .map((announcement, i, xs) => {
              const id = game.announcements.length - i;

              if (announcement.type == "Action") {
                const a = announcement.content;
                return (
                  <React.Fragment key={id}>
                    <motion.div
                      layoutId={`left-${id}`}
                      className="text-slate-400 w-full text-right"
                    >
                      <span className="text-xl text-white">
                        P{a.player_asking + 1}
                      </span>{" "}
                      asked{" "}
                      <span className="text-xl text-white">
                        P{a.player_asked + 1}
                      </span>{" "}
                      for
                    </motion.div>
                    <div
                      className="relative"
                      style={{ zIndex: game.announcements.length - i }}
                    >
                      <Hand cards={inAllSuits(a.asked_for).slice(0, 4)} />
                    </div>
                    <motion.div
                      layoutId={`right-${id}`}
                      className="text-center flex items-center justify-center space-x-1"
                    >
                      <span className="text-slate-400">got</span>
                      {a.response.type == "GoFish" ? (
                        <span className="text-3xl">🐟</span>
                      ) : (
                        <span className="flex place-items-center space-x-1 text-xl">
                          <span>{a.response.content.count}</span>
                          <img className="w-4 h-6" src={card.BACKS[0]} />
                        </span>
                      )}
                    </motion.div>
                  </React.Fragment>
                );
              } else {
                const a = announcement.content;

                return (
                  <React.Fragment key={id}>
                    <motion.div
                      layoutId={`left-${id}`}
                      className="text-slate-400 text-right w-full"
                    >
                      <span className="text-xl text-white">
                        P{a.player + 1}
                      </span>{" "}
                      got all{" "}
                    </motion.div>
                    <div
                      className="relative"
                      style={{ zIndex: game.announcements.length - i }}
                    >
                      <Hand cards={inAllSuits(a.book).slice(0, 4)} />
                    </div>
                    <motion.div
                      layoutId={`right-${id}`}
                      className="text-center flex items-center justify-center space-x-1"
                    >
                      {/* <span className="text-slate-400">got</span>
                      {a.response.type == "GoFish" ? (
                        <span className="text-3xl">🐟</span>
                      ) : (
                        <span className="flex place-items-center space-x-1 text-xl">
                          <span>{a.response.content.count}</span>
                          <img className="w-4 h-6" src={card.BACKS[0]} />
                        </span>
                      )} */}
                    </motion.div>
                  </React.Fragment>
                );
              }
            })}
        </div>
      </div>
      <div className="flex flex-col p-10 space-y-10 items-center w-full">
        <div
          className="space-x-10 grid w-full"
          style={{ gridTemplateColumns: `repeat(${game.players.length}, 1fr)` }}
        >
          {game.players.map((p, pid) => (
            <div key={pid} className="flex flex-col items-center">
              <motion.div
                layout
                className="text-center text-xl font-medium whitespace-nowrap"
              >
                Player {pid + 1} ({p.books.length})
              </motion.div>
              <div>
                <Hand cards={p.hand} hidden={pid != 0 && true} />
              </div>
              {pid == game.stage.content.who_next &&
                game.stage.name == "Playing" && (
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
        <div>
          <Hand cards={game.pool} hidden />
        </div>
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

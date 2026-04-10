"use client";

import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  const [status, setStatus] = useState<string>("Aguardando...");
  const [lastResult, setLastResult] = useState<string>("");

  useEffect(() => {
    let unlistenWakeWord: () => void;
    let unlistenSTT: () => void;

    async function setupListeners() {
      // Listen for the global events emitted by the Rust backend
      const unlistenWW = await listen("WAKE_WORD_DETECTED", () => {
        setStatus("Palavra de ativação detectada!");
        // Reset status after 3 seconds
        setTimeout(() => setStatus("Aguardando..."), 3000);
      });
      unlistenWakeWord = unlistenWW;

      const unlistenResult = await listen<string>("STT_RESULT", (event) => {
        setLastResult(event.payload);
      });
      unlistenSTT = unlistenResult;
    }

    setupListeners();

    return () => {
      if (unlistenWakeWord) unlistenWakeWord();
      if (unlistenSTT) unlistenSTT();
    };
  }, []);

  const triggerWakeWord = () => invoke("test_wake_word");
  const triggerSTT = () => invoke("test_stt", { result: "Exemplo de resultado STT" });

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-zinc-50 dark:bg-zinc-950">
      <div className="z-10 max-w-5xl w-full flex flex-col gap-8 p-12 rounded-3xl bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 shadow-xl">
        <div className="space-y-2">
          <h1 className="text-5xl font-black tracking-tighter text-zinc-900 dark:text-zinc-50">
            Mila <span className="text-blue-600">Core</span>
          </h1>
          <p className="text-zinc-500 dark:text-zinc-400 font-medium">
            Tauri v2 + Next.js Base Structure
          </p>
        </div>

        <div className="grid gap-6 py-4">
          <div className="p-6 rounded-2xl bg-zinc-100/50 dark:bg-zinc-800/50 border border-zinc-200 dark:border-zinc-700">
            <p className="text-sm font-bold uppercase tracking-widest text-zinc-400 mb-1">Status do Sistema</p>
            <p className="text-2xl font-bold text-blue-600 dark:text-blue-400">{status}</p>
          </div>

          <div className="p-6 rounded-2xl bg-zinc-100/50 dark:bg-zinc-800/50 border border-zinc-200 dark:border-zinc-700">
            <p className="text-sm font-bold uppercase tracking-widest text-zinc-400 mb-1">Último Resultado STT</p>
            <p className="text-2xl font-medium text-zinc-800 dark:text-zinc-200 italic">
              {lastResult ? `"${lastResult}"` : "Nenhum resultado ainda"}
            </p>
          </div>
        </div>
        
        <div className="flex flex-wrap gap-4">
          <button 
            onClick={triggerWakeWord}
            className="px-8 py-4 bg-zinc-900 dark:bg-zinc-50 text-white dark:text-zinc-900 font-bold rounded-2xl transition hover:scale-105 active:scale-95 duration-200 shadow-xl"
          >
            Simular Wake Word
          </button>
          <button 
            onClick={triggerSTT}
            className="px-8 py-4 border-2 border-zinc-900 dark:border-zinc-50 text-zinc-900 dark:text-zinc-50 font-bold rounded-2xl transition hover:bg-zinc-900/5 dark:hover:bg-zinc-50/5 hover:scale-105 active:scale-95 duration-200"
          >
            Simular STT
          </button>
        </div>
      </div>
    </main>
  );
}

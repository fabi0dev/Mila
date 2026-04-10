'use client';

import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  const [status, setStatus] = useState("Aguardando 'Mila'...");
  const [lastTranscript, setLastTranscript] = useState("");
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    const unlistenWake = listen("WAKE_WORD_DETECTED", () => {
      setStatus("Ouvindo...");
      setIsListening(true);
    });

    const unlistenSTT = listen("STT_RESULT", (event: { payload: string }) => {
      setStatus("Aguardando 'Mila'...");
      setIsListening(false);
      setLastTranscript(event.payload);
    });

    return () => {
      unlistenWake.then(f => f());
      unlistenSTT.then(f => f());
    };
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-8 bg-[#0a0a0a] text-white">
      <div className="z-10 w-full max-w-md flex flex-col items-center space-y-8">
        <h1 className="text-4xl font-bold bg-gradient-to-r from-purple-400 to-pink-600 bg-clip-text text-transparent">
          Mila AI
        </h1>

        <div className={`w-32 h-32 rounded-full flex items-center justify-center transition-all duration-500 shadow-[0_0_20px_rgba(0,0,0,0.5)] ${
          isListening ? 'bg-pink-500 shadow-[0_0_60px_rgba(236,72,153,0.5)] scale-110' : 'bg-zinc-800'
        }`}>
          <div className="w-24 h-24 rounded-full bg-black flex items-center justify-center overflow-hidden border border-zinc-700/50">
             <div className={`w-full h-1 bg-pink-500 transition-all ${isListening ? 'animate-pulse' : 'opacity-20'}`} />
          </div>
        </div>

        <div className="text-center space-y-2">
          <p className="text-zinc-400 font-medium tracking-wide uppercase text-xs">Status do Sistema</p>
          <p className="text-2xl font-bold tracking-tight">{status}</p>
        </div>

        {lastTranscript && (
          <div className="w-full p-6 bg-zinc-900/40 border border-zinc-800/50 rounded-3xl backdrop-blur-xl animate-in fade-in slide-in-from-bottom-4 duration-500">
            <p className="text-[10px] text-zinc-500 uppercase tracking-widest font-bold mb-3 border-b border-zinc-800/50 pb-2">Transcrição Recente</p>
            <p className="text-zinc-200 text-lg leading-relaxed">"{lastTranscript}"</p>
          </div>
        )}

        <div className="flex space-x-4 pt-8">
          <button 
            onClick={() => invoke("test_wake_word")}
            className="px-5 py-2.5 bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 rounded-xl text-xs font-semibold text-zinc-400 transition-all active:scale-95"
          >
            Simular Wake Word
          </button>
          <button 
            onClick={() => invoke("test_stt", { result: "Olá! Como posso ajudar você hoje?" })}
            className="px-5 py-2.5 bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 rounded-xl text-xs font-semibold text-zinc-400 transition-all active:scale-95"
          >
            Simular Resposta
          </button>
        </div>

        <p className="fixed bottom-8 text-[10px] text-zinc-600 tracking-widest uppercase">
          Mila • Local & Open Source
        </p>
      </div>
    </main>
  );
}

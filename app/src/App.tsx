import { useEffect } from "react";
import { Routes, Route } from "react-router-dom";
import { listen } from "@tauri-apps/api/event";
import { useQueryClient } from "@tanstack/react-query";
import { Layout } from "./components/Layout";
import { Inbox } from "./pages/Inbox";
import { Library } from "./pages/Library";
import { Neglected } from "./pages/Neglected";
import { Purgatory } from "./pages/Purgatory";
import { Settings } from "./pages/Settings";

function App() {
  const queryClient = useQueryClient();

  useEffect(() => {
    const unlisten = listen<{ imported: number; duplicates: number }>(
      "tabs-imported",
      () => {
        queryClient.invalidateQueries({ queryKey: ["tabs"] });
      },
    );

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [queryClient]);

  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Inbox />} />
        <Route path="/library" element={<Library />} />
        <Route path="/neglected" element={<Neglected />} />
        <Route path="/purgatory" element={<Purgatory />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </Layout>
  );
}

export default App;

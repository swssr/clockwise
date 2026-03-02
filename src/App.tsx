import { useEffect, useId, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";

type Timezone = string;

const DEFAULT_TZS: Timezone[] = [
  "America/Chicago",
  "America/New_York",
  "Asia/Manila",
];

async function fetchTimezones(): Promise<Timezone[]> {
  return await invoke("list_timezones");
}

async function saveSelectedTimezones(tz_names: string[]) {
  await invoke("set_selected_timezones", { tz_names });
}

function App() {
  const [timezones, setTimezones] = useState<Timezone[]>([]);
  const [selected, setSelected] = useState<Timezone[]>(DEFAULT_TZS);

  useEffect(() => {
    fetchTimezones().then(setTimezones);
  }, []);

  useEffect(() => {
    saveSelectedTimezones(selected).catch(console.error);
  }, [selected]);

  const renderZones = timezones
    .map((x) => ({ name: x, id: x }))
    .sort((a, b) => {
      const aSelected = selected.includes(a.name);
      const bSelected = selected.includes(b.name);

      if (aSelected && !bSelected) return -1;
      if (!aSelected && bSelected) return 1;
      return 0;
    });

  const handleSelected = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const values = Array.from(e.target.selectedOptions).map((o) => o.value);
    setSelected(values);
  };

  return (
    <div className="container">
      <select
        name="timezone"
        id="timezone-select"
        multiple
        size={12}
        value={selected}
        onChange={handleSelected}
      >
        {renderZones.map((zone) => {
          return (
            <option id={zone.id} value={zone.name}>
              {zone.name}
            </option>
          );
        })}
      </select>
    </div>
  );
}

export default App;

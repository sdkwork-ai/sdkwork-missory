import type { ReactNode } from "react";

export function SectionCard({ title, action, children }: {
  title: string;
  action?: ReactNode;
  children: ReactNode;
}) {
  return (
    <section className="sdk-card" style={{ display: "grid", gap: 12 }}>
      <header style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
        <h3 style={{ margin: 0, fontSize: 15 }}>{title}</h3>
        {action}
      </header>
      {children}
    </section>
  );
}

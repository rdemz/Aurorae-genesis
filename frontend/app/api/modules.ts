// app/api/modules.ts
export async function GET() {
  const modules = [
    {
      name: "DreamEngine",
      purpose: "Génère des rêves et visions IA",
      created_at: "2025-04-07"
    },
    {
      name: "GuardianSentinel",
      purpose: "Surveillance des modules et autoréparation",
      created_at: "2025-04-06"
    }
  ];

  return Response.json(modules);
}

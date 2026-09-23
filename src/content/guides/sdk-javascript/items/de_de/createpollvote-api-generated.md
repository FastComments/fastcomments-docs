---
Erfasse eine Stimme zu einer Umfrage oder verschiebe eine bereits vorhandene Stimme zu einer anderen Option. Ein Wähler hat höchstens eine Stimme pro  
Umfrage, sodass ein erneuter Aufruf für denselben Wähler seine Stimme verschiebt, anstatt eine neue hinzuzufügen.

Dies beachtet die Umfrageeinstellungen der Seite: Wenn das Abstimmen nur für angemeldete Benutzer erlaubt ist, wird eine Stimme nur mit einer  
anonUserId abgelehnt, und anonyme Stimmen werden pro IP und Umfrage rate‑limitiert.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| createPollVoteBody | CreatePollVoteBody | Ja |  |

## Antwort

Rückgabe: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createPollVote Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId ist optional und hier weggelassen
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]

---
Bearbeiten Sie eine Umfrage direkt, ohne die Stimmen zu verlieren: Ändern Sie die Frage, benennen Sie eine Option um, schließen oder öffnen Sie sie erneut,
oder ändern Sie, wer die Wähler sehen darf. Optionen werden über die id adressiert – um sie hinzuzufügen, zu entfernen oder neu zu ordnen, senden Sie die vollständige Liste per PUT.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| commentPollPatch | CommentPollPatch | Ja |  |

## Antwort

Gibt zurück: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'patchPoll Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]
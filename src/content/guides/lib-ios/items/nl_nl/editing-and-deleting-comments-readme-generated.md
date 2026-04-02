### Bewerken

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

De server rendeert de HTML opnieuw. De lokale opmerking wordt automatisch bijgewerkt.

### Verwijderen

```swift
try await sdk.deleteComment(commentId: commentId)
```

Het verwijderen van een opmerking verwijdert ook de onderliggende reacties uit de lokale boom.

Beide acties zijn beschikbaar via het contextmenu van de opmerking in de UI wanneer de huidige gebruiker de auteur van de opmerking is (of een sitebeheerder).
Wywoływane, gdy komentarz zostanie przypięty.

### Kontekst, który otrzymuje agent

- Przypięty komentarz.
- Opcjonalne informacje o wątku / historii użytkownika / kontekście strony, zgodnie z konfiguracją.

### Kto to wywołuje

- Moderator klikający akcję przypięcia na stronie moderacji lub w widżecie komentarza.
- Agent wywołujący [`pin_comment`](#tools-overview).

Zapobieganie pętlom: zdarzenia przypięcia pochodzące od agenta nie uruchamiają żadnych wyzwalaczy agenta. Przypięcie wykonane przez agenta przerywa wszelkie wywołania agentów dla tego zdarzenia, nie tylko dla agenta, który je zainicjował.

### Uwaga dotycząca pary

Zdarzenia przypięcia i odpięcia są oddzielnymi wyzwalaczami. Subskrybuj oba, jeśli chcesz symetrycznego zachowania. Zobacz [Wyzwalacz: Komentarz odpięty](#trigger-comment-unpin).

---
Sobald FastComments in Ihrem LMS registriert ist, fügen Dozierende es zu Kursen auf die gleiche Weise hinzu wie jedes andere LTI-External-Tool.

#### D2L Brightspace

Im Inhaltsbereich eines Kurses:

1. Klicken Sie auf **Vorhandene Aktivitäten hinzufügen** > **Externe Lernwerkzeuge**.
2. Wählen Sie **FastComments** aus der Liste.
3. Das Tool erscheint als Thema im Inhaltsbereich. Öffnen Sie es einmal, um den Kommentarthread für diese Ressource zu initialisieren.

#### Moodle

In einem Kurs:

1. Schalten Sie den **Bearbeitungsmodus** ein.
2. Klicken Sie in dem Abschnitt, in dem Sie Kommentare wünschen, auf **Aktivität oder Ressource hinzufügen**.
3. Wählen Sie **FastComments** aus dem Aktivitätsauswähler.
4. Speichern. Studierende sehen den Kommentarthread eingebettet im Abschnitt.

#### Blackboard Learn

In einem Kurs:

1. Navigieren Sie zu einem Inhaltsbereich.
2. Klicken Sie auf **Inhalte erstellen** > **FastComments** (unter 'Lernwerkzeuge').
3. Vergeben Sie einen Namen und bestätigen Sie.

#### Sakai

Site-Verwalter fügen das Tool über **Site-Info** > **Tools verwalten** > scrollen Sie zu **Externe Tools** > wählen Sie **FastComments** > **Weiter** hinzu.

#### How Threads Are Scoped

FastComments erstellt einen separaten Kommentarthread pro **(LMS-Instanz, Kurs, Ressourcenlink)**. Das bedeutet:

- Zwei verschiedene Kurse im selben LMS erhalten separate Threads, selbst wenn sie denselben Tool-Namen verwenden.
- Dasselbe FastComments-Tool, das an zwei Stellen innerhalb eines Kurses verwendet wird, erzeugt zwei Threads.
- Zwei verschiedene Brightspace-Installationen unter demselben FastComments-Konto erhalten separate Threads – der Hostname des LMS ist Teil des Thread-Identifikators.

#### SSO

Studierende sehen keinen Anmeldebildschirm. Das LMS übermittelt ihre Identität (Name, E-Mail, Avatar, Rolle) an FastComments über den LTI-Launch, und FastComments meldet sie automatisch an. Ihre Kommentare werden ihrem LMS-Konto zugeordnet.

Benutzer mit den LMS-Rollen **Instructor** oder **Administrator** werden automatisch den FastComments-Moderator-/Admin-Rollen für den Thread zugeordnet.
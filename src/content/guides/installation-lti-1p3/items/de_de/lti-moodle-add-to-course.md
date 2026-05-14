Dieser Leitfaden behandelt das Hinzufügen von FastComments zu einem Moodle 4.x-Kurs, nachdem ein Website-Administrator das Tool registriert und so eingestellt hat, dass es im Aktivitätswähler angezeigt wird. Wenn FastComments noch nicht registriert ist, lesen Sie zuerst die Moodle-Registrierungsanleitung.

#### Öffnen Sie den Kurs im Bearbeitungsmodus

1. Melden Sie sich in Moodle als Editing Teacher (oder höher) für den Kurs an.
2. Öffnen Sie den Kurs.
3. Schalten Sie den **Edit mode** über den Schalter in der oberen rechten Ecke des Kurskopfs ein.

Moodle 4.x hat das alte Dropdown "Add an activity or resource" aus 3.x durch einen vollbildigen Aktivitätswähler ersetzt. Moodle 4.5 behält denselben Wähler bei, fügt jedoch eine Zeile mit markierten/Favoriten oben hinzu, sodass das Anheften von FastComments es später in anderen Abschnitten schneller erreichbar macht.

#### Fügen Sie die FastComments-Aktivität hinzu

1. Scrollen Sie zu dem Kursabschnitt (Thema oder Woche), in dem die Diskussion stattfinden soll.
2. Klicken Sie am Ende dieses Abschnitts auf **Add an activity or resource**.
3. Wählen Sie im Wählerdialog **FastComments** aus. Wenn Sie es nicht sehen, springen Sie zum Abschnitt "Gotchas" weiter unten.

Das Formular für die Aktivitätseinstellungen öffnet sich. Die Felder, die wichtig sind:

- **Activity name** (erforderlich). Wird auf der Kursseite und im Notenbuch angezeigt. Beispiel: `Week 3 Discussion`.
- **Activity description**. Optionaler Einführungstext, der über dem Kommentarthread gerendert wird.
- **Show description on course page**. Aktivieren Sie dies, wenn die Beschreibung ohne Öffnen der Aktivität sichtbar sein soll.
- **Preconfigured tool**. Auf `FastComments` einstellen (wird beim Start aus dem Wähler automatisch ausgewählt). Nicht ändern.
- **Launch container**. Auf **New window** einstellen. Siehe den Abschnitt "Gotchas", warum "Same window" in einigen Moodle-Installationen Probleme verursacht.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Freilassen. Dynamic Registration hat diese auf Site-Ebene verwaltet.

Scrollen Sie nach unten und klicken Sie auf **Save and return to course** (oder **Save and display**, um die Aktivität sofort zu öffnen).

Die Aktivität erscheint als Eintrag in der Sektion mit dem FastComments-Symbol. Studierende klicken auf den Eintrag, um den Kommentarthread zu öffnen.

#### FastComments inline im Editor einbetten

Für einen Thread innerhalb einer Seite (Page), eines Buchkapitels (Book), einer Lektion (Lesson) oder jeder anderen Ressource, die den Atto- oder TinyMCE-Editor verwendet:

1. Öffnen Sie die Ressource im Bearbeitungsmodus.
2. Platzieren Sie den Cursor an der Stelle, an der der Thread erscheinen soll.
3. Klicken Sie in der Editor-Symbolleiste auf die **LTI** / **External tool**-Schaltfläche. In Atto ist sie mit "Insert LTI Advantage content" beschriftet. In TinyMCE (Standard in Moodle 4.3+) befindet sie sich im **More**-Menü als **External tools**.
4. Wählen Sie in der Tool-Liste **FastComments** aus.
5. FastComments öffnet einen Deep-Link-Auswahl-Dialog. Bestätigen Sie den Thread-Titel und klicken Sie auf **Embed**.
6. Der Editor fügt einen LTI-Platzhalterblock ein. Speichern Sie die Ressource.

Jede eingebettete Instanz ist ein eigener Thread, der anhand der Deep-Link-Content-Item-ID unterschieden wird. Eine Seite mit drei FastComments-Einbettungen erhält also drei unabhängige Threads.

#### Zugriffsbeschränkungen und Gruppeneinstellungen

Die Standard-Aktivitätseinstellungen von Moodle gelten für FastComments-Aktivitäten:

- **Common module settings** > **Group mode**. Das Setzen auf **Separate groups** oder **Visible groups** teilt FastComments nicht automatisch in gruppenspezifische Threads auf. Moodles Gruppenmodus filtert nur das Notenbuch und die Mitgliederliste. Um pro Gruppe einen separaten Thread zu betreiben, fügen Sie pro Gruppe eine FastComments-Aktivität hinzu und verwenden Sie **Restrict access**, um jede einzelne zu begrenzen.
- **Restrict access** > **Add restriction**. Unterstützt die Standardbedingungen von Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile** und verschachtelte Einschränkungssets. Verwenden Sie **Group**, um eine FastComments-Aktivität auf eine einzelne Gruppe zu beschränken.
- **Activity completion**. Auf **Students must view this activity to complete it** setzen, wenn Sie die Erfassung des Abschlusses wünschen. FastComments meldet derzeit kein Abschlussereignis an Moodle zurück, abgesehen vom Launch.

#### Role Mapping

FastComments liest den LTI `roles` Claim, den Moodle bei jedem Start sendet, und bildet ihn wie folgt ab:

- Moodle **Manager** or **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** or **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> Nur-Lesezugriff

Admins können jeden Kommentar löschen, Benutzer sperren und Thread-Einstellungen bearbeiten. Moderatoren können innerhalb des Threads, in den sie gestartet wurden, Kommentare löschen und genehmigen. Benutzerdefinierte Moodle-Rollen erben die Abbildung des Archetyps, von dem sie geklont wurden.

#### Was Studierende sehen

Studierende klicken die FastComments-Aktivität an (oder scrollen zu dem eingebetteten Block innerhalb einer Seite oder eines Buchs). Moodle sendet ihre Identität über den LTI-Launch an FastComments:

- Kein Login-Bildschirm. FastComments meldet sie mit dem Moodle-Konto an.
- Anzeigename, E-Mail und Avatar stammen aus Moodle.
- Der Thread ist auf `(Moodle site, course, resource link ID)` beschränkt, sodass dieselbe Aktivität, die in einen anderen Kurs kopiert wird, einen neuen Thread erhält.
- Verschachtelte Antworten, Abstimmungen und Benachrichtigungen funktionieren wie in einem eigenständigen FastComments-Thread.

#### Moodle-Tücken

**FastComments fehlt im Aktivitätswähler.** Der Website-Administrator hat das Tool registriert, aber nicht **Tool configuration usage** auf **Show in activity chooser and as a preconfigured tool** gesetzt. Beheben Sie dies unter **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > Zahnrad-Symbol auf der FastComments-Kachel.

**Der Start schlägt fehl oder zeigt einen leeren Rahmen, wenn auf "Same window" gesetzt.** Moodles Sitzungs-Cookies verwenden standardmäßig `SameSite=Lax`, und einige Browser entfernen sie bei dem Cross-Site-POST, das LTI 1.3 verwendet, um von FastComments zurückzukehren. Stellen Sie **Launch container** auf **New window** für die Aktivität ein. Dies ist eine harte Anforderung für eingebettete FastComments innerhalb einer Seite oder eines Buchs, da der aus dem Editor eingebettete Startpfad immer ein neues Fenster öffnet.

**Der `iss` Claim ist die Moodle-Site-URL, nicht eine Tenant-ID.** FastComments verwendet die Moodle-Site-URL (den `wwwroot`-Konfigurationswert) als LTI-Issuer. Wenn Ihre Moodle-Instanz auf eine neue Domain umzieht oder Sie `wwwroot` ändern, bleiben vorhandene FastComments-Threads an den alten Issuer gebunden und stimmen nicht mit neuen Starts überein. Registrieren Sie das Tool gegen die neue URL neu und migrieren Sie Threads über das FastComments-Admin-Interface, falls nötig.

**Aktivitäts-Backup und Wiederherstellung.** Das Sichern eines Kurses und Wiederherstellen in einen neuen Kurs erzeugt neue Resource-Link-IDs, sodass die wiederhergestellten FastComments-Aktivitäten mit leeren Threads beginnen. Der ursprüngliche Kurs behält die ursprünglichen Threads. Dies ist beabsichtigtes Verhalten, kein Fehler.

**Moodle 4.5 TinyMCE als Standard.** Moodle 4.5 wird bei Neuinstallationen mit TinyMCE als Standardeditor ausgeliefert. Die Schaltfläche External tool befindet sich im **More** (`...`) Menü statt in der Hauptsymbolleiste. Ältere Seiten, die von 4.1 aktualisiert wurden, behalten Atto bei, sofern ein Administrator nicht den Standard geändert hat.
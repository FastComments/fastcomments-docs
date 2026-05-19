Diese Anleitung behandelt das Hinzufügen von FastComments zu einem Moodle 4.x-Kurs, nachdem ein Site-Administrator das Tool registriert und so eingestellt hat, dass es im Aktivitätsauswahlfenster angezeigt wird. Falls FastComments noch nicht registriert ist, siehe zuerst die Moodle-Registrierungsanleitung.

#### Kurs im Bearbeitungsmodus öffnen

1. Melden Sie sich in Moodle als Lehrkraft mit Bearbeitungsrechten (oder höher) für den Kurs an.
2. Öffnen Sie den Kurs.
3. Schalten Sie den **Bearbeitungsmodus** mithilfe des Schalters oben rechts in der Kurskopfzeile ein.

Moodle 4.x hat das alte Dropdown „Add an activity or resource“ aus 3.x durch einen vollbildigen Aktivitätsauswahl-Dialog ersetzt. Moodle 4.5 behält denselben Auswahl-Dialog bei, fügt jedoch eine Zeile mit Favoriten/Sternen oben hinzu. Das Anpinnen von FastComments macht es später schneller erreichbar.

#### Die FastComments-Aktivität hinzufügen

1. Scrollen Sie zu dem Kursabschnitt (Thema oder Woche), in dem die Diskussion stattfinden soll.
2. Klicken Sie am Ende dieses Abschnitts auf **Add an activity or resource**.
3. Wählen Sie im Auswahl-Dialog **FastComments** aus. Wenn Sie es nicht sehen, springen Sie zum Abschnitt „Gotchas“ weiter unten.

Das Formular für die Aktivitätseinstellungen öffnet sich. Die relevanten Felder:

- **Activity name** (erforderlich). Wird auf der Kursseite und im Notenbuch angezeigt. Beispiel: `Week 3 Discussion`.
- **Activity description**. Optionaler Einführungstext, der über dem Kommentarthread angezeigt wird.
- **Show description on course page**. Aktivieren, wenn die Beschreibung ohne Öffnen der Aktivität sichtbar sein soll.
- **Preconfigured tool**. Auf `FastComments` setzen (wird beim Start aus dem Auswahl-Dialog automatisch ausgewählt). Nicht ändern.
- **Launch container**. Auf **New window** setzen. Siehe den Abschnitt „Gotchas“, warum „Same window“ in einigen Moodle-Installationen Probleme verursacht.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Leer lassen. Die dynamische Registrierung hat diese auf Site-Ebene behandelt.

Scrollen Sie nach unten und klicken Sie auf **Save and return to course** (oder **Save and display**, um die Aktivität sofort zu öffnen).

Die Aktivität erscheint als Eintrag im Abschnitt mit dem FastComments-Icon. Studierende klicken auf den Eintrag, um den Kommentarthread zu öffnen.

#### FastComments inline mit dem Editor einbetten

Für einen Thread innerhalb einer Seite, eines Buchkapitels, einer Lektion oder einer anderen Ressource, die den Atto- oder TinyMCE-Editor verwendet:

1. Öffnen Sie die Ressource im Bearbeitungsmodus.
2. Platzieren Sie den Cursor an der Stelle, an der der Thread erscheinen soll.
3. Klicken Sie in der Editor-Symbolleiste auf die Schaltfläche **LTI** / **External tool**. In Atto ist sie mit „Insert LTI Advantage content“ beschriftet. In TinyMCE (Standard in Moodle 4.3+) befindet sie sich im **More**-Menü als **External tools**.
4. Wählen Sie **FastComments** aus der Tool-Liste.
5. FastComments öffnet einen Deep-Linking-Auswahl-Dialog. Bestätigen Sie den Thread-Titel und klicken Sie auf **Embed**.
6. Der Editor fügt einen LTI-Platzhalterblock ein. Speichern Sie die Ressource.

Jede eingebettete Instanz ist ein eigener Thread, der über die Deep-Link-Content-Item-ID eindeutig ist. Eine Seite mit drei FastComments-Einbettungen erzeugt also drei unabhängige Threads.

#### Zugriffsbeschränkungen und Gruppeneinstellungen

Die standardmäßigen Moodle-Aktivitätseinstellungen gelten für FastComments-Aktivitäten:

- **Common module settings** > **Group mode**. Das Setzen auf **Separate groups** oder **Visible groups** teilt FastComments nicht automatisch in gruppenspezifische Threads auf. Moodles Gruppenmodus filtert nur das Notenbuch und die Mitgliederliste. Um pro Gruppe einen separaten Thread zu betreiben, fügen Sie pro Gruppe eine FastComments-Aktivität hinzu und verwenden Sie **Restrict access**, um jede einzelne zu begrenzen.
- **Restrict access** > **Add restriction**. Unterstützt die üblichen Moodle-Bedingungen: **Date**, **Grade**, **Group**, **Grouping**, **User profile** und verschachtelte Einschränkungsgruppen. Verwenden Sie **Group**, um eine FastComments-Aktivität auf eine einzelne Gruppe zu beschränken.
- **Activity completion**. Auf **Students must view this activity to complete it** setzen, wenn Sie die Abschlussverfolgung wünschen. FastComments meldet derzeit kein Abschlussereignis an Moodle zurück, außer dem Start.

#### Rollen-Mapping

FastComments liest die LTI-`roles`-Claim, die Moodle bei jedem Start sendet, und mappt sie wie folgt:

- Moodle **Manager** oder **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** oder **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> Nur-Lesezugriff

Admins können jeden Kommentar löschen, Benutzer sperren und Thread-Einstellungen bearbeiten. Moderatoren können Kommentare innerhalb des Threads, in den sie gestartet wurden, löschen und genehmigen. Benutzerdefinierte Moodle-Rollen erben die Zuordnung des Archetyps, von dem sie geklont wurden.

#### Was Studierende sehen

Studierende klicken auf die FastComments-Aktivität (oder scrollen zum eingebetteten Block innerhalb einer Seite oder eines Buchs). Moodle übermittelt ihre Identität an FastComments via dem LTI-Start:

- Kein Login-Bildschirm. FastComments meldet sie mit dem Moodle-Konto an.
- Anzeigename, E-Mail und Avatar kommen aus Moodle.
- Der Thread ist auf (Moodle-Site, Kurs, Ressourcen-Link-ID) begrenzt, sodass die gleiche Aktivität, die in einen anderen Kurs dupliziert wird, einen neuen Thread erhält.
- Threaded Replies, Voting und Benachrichtigungen funktionieren wie in einem eigenständigen FastComments-Thread.

#### Öffentlichen Zugriff einschränken (empfohlen)

Standardmäßig sind FastComments-Kommentardaten öffentlich lesbar. Jeder, der die URL oder die API-Endpunkt eines Threads errät, kann dessen Kommentare ansehen, auch außerhalb von Moodle. Für Kursdiskussionen möchten Sie das Anzeigen nahezu sicher nur auf eingeschriebene Studierende beschränken.

Öffnen Sie Ihre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Seite zur Anpassung des Widgets</a> und erstellen Sie eine Regel mit aktivierter Option **Require SSO To View Comments**, setzen Sie dann das Sicherheitslevel auf **Secure SSO**, damit Threads nur über den signierten LTI-Start geladen werden können.

Siehe [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) für die vollständige Schritt-für-Schritt-Anleitung, einschließlich wie Sie die Regel auf eine einzelne Domain oder Seite einschränken.

#### Moodle Gotchas

**FastComments fehlt im Aktivitätsauswahl-Dialog.** Der Site-Administrator hat das Tool registriert, aber die Option **Tool configuration usage** nicht auf **Show in activity chooser and as a preconfigured tool** gesetzt. Beheben Sie das unter **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > Zahnrad-Symbol auf der FastComments-Kachel.

**Start schlägt fehl oder zeigt einen leeren Rahmen an, wenn auf „Same window“ gesetzt.** Moodles Sitzungs-Cookies verwenden standardmäßig `SameSite=Lax`, und einige Browser entfernen sie beim Cross-Site-POST, den LTI 1.3 für die Rückkehr von FastComments verwendet. Setzen Sie **Launch container** auf **New window** in der Aktivität. Dies ist eine harte Voraussetzung für eingebettete FastComments innerhalb einer Seite oder eines Buchs, da der im Editor eingebettete Startpfad immer ein neues Fenster öffnet.

**Die `iss`-Claim ist die Moodle-Site-URL, nicht eine Tenant-ID.** FastComments verwendet die Moodle-Site-URL (den `wwwroot`-Konfigurationswert) als LTI-Issuer. Wenn Ihre Moodle-Instanz auf eine neue Domain umzieht oder Sie `wwwroot` ändern, bleiben bestehende FastComments-Threads an den alten Issuer gebunden und stimmen nicht mit neuen Starts überein. Registrieren Sie das Tool gegen die neue URL neu und migrieren Sie Threads über das FastComments-Admin, falls nötig.

**Backup und Wiederherstellung von Aktivitäten.** Das Sichern eines Kurses und das Wiederherstellen in einen neuen Kurs erzeugt neue Ressourcen-Link-IDs, sodass die wiederhergestellten FastComments-Aktivitäten mit leeren Threads beginnen. Der ursprüngliche Kurs behält die Original-Threads. Dies ist beabsichtigtes Verhalten, kein Fehler.

**Moodle 4.5 TinyMCE als Standard.** Moodle 4.5 wird bei neuen Installationen mit TinyMCE als Standard-Editor ausgeliefert. Die Schaltfläche für External tools befindet sich im **More** (`...`) Menü und nicht in der Hauptsymbolleiste. Ältere Sites, die von 4.1 aktualisiert wurden, behalten Atto, es sei denn, ein Administrator hat den Standard geändert.
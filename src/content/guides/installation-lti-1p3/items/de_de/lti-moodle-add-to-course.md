Diese Anleitung behandelt das Hinzufügen von FastComments zu einem Moodle 4.x-Kurs, nachdem ein Website-Administrator das Tool registriert und so konfiguriert hat, dass es in der Aktivitätsauswahl angezeigt wird. Wenn FastComments noch nicht registriert ist, lesen Sie zuerst die Moodle-Registrierungsanleitung.

#### Kurs im Bearbeitungsmodus öffnen

1. Melden Sie sich in Moodle als Lehrkraft mit Bearbeitungsrecht (oder höher) für den Kurs an.
2. Öffnen Sie den Kurs.
3. Aktivieren Sie den **Bearbeitungsmodus** über den Schalter oben rechts in der Kurskopfleiste.

Moodle 4.x hat das bisherige Dropdown „Aktivität oder Material hinzufügen" aus 3.x durch einen Vollbild-Dialog zur Aktivitätsauswahl ersetzt. Moodle 4.5 behält dieselbe Auswahl bei, fügt oben jedoch eine Zeile mit Favoriten (Sternchen) hinzu, sodass das einmalige Anpinnen von FastComments den Zugriff in späteren Abschnitten beschleunigt.

#### FastComments-Aktivität hinzufügen

1. Scrollen Sie zu dem Kursabschnitt (Thema oder Woche), in dem die Diskussion erscheinen soll.
2. Klicken Sie am unteren Rand dieses Abschnitts auf **Aktivität oder Material hinzufügen**.
3. Wählen Sie im Auswahl-Dialog **FastComments**. Wenn Sie es nicht sehen, springen Sie zum Abschnitt „Gotchas" unten.

Das Einstellungsformular der Aktivität öffnet sich. Die wichtigsten Felder:

- **Name der Aktivität** (Pflichtfeld). Wird auf der Kursseite und im Bewertungsbereich angezeigt. Beispiel: `Week 3 Discussion`.
- **Beschreibung der Aktivität**. Optionaler Einführungstext, der über dem Kommentar-Thread angezeigt wird.
- **Beschreibung auf der Kursseite anzeigen**. Aktivieren Sie dies, wenn die Beschreibung sichtbar sein soll, ohne dass man in die Aktivität klicken muss.
- **Vorkonfiguriertes Tool**. Auf `FastComments` gesetzt (automatisch ausgewählt, wenn aus der Auswahl gestartet). Nicht ändern.
- **Start-Container**. Auf **Neues Fenster** setzen. Warum „Gleiches Fenster" in manchen Moodle-Installationen Probleme verursacht, erfahren Sie im Abschnitt „Gotchas".
- **Tool-URL**, **Öffentlicher Schlüssel**, **Geteiltes Geheimnis**, **Benutzerdefinierte Parameter**. Leer lassen. Die dynamische Registrierung hat diese auf Site-Ebene behandelt.

Scrollen Sie nach unten und klicken Sie auf **Speichern und zum Kurs zurückkehren** (oder **Speichern und anzeigen**, um die Aktivität sofort zu öffnen).

Die Aktivität erscheint als Zeile im Abschnitt mit dem FastComments-Symbol. Studierende klicken auf die Zeile, um den Kommentar-Thread zu öffnen.

#### FastComments inline über den Editor einbetten

Für einen Thread innerhalb einer Seite, eines Buchkapitels, einer Lektion oder eines anderen Materials, das den Atto- oder TinyMCE-Editor verwendet:

1. Öffnen Sie das Material im Bearbeitungsmodus.
2. Platzieren Sie den Cursor an der Stelle, an der der Thread erscheinen soll.
3. Klicken Sie in der Editor-Symbolleiste auf die Schaltfläche **LTI** / **Externes Tool**. In Atto ist sie mit „Insert LTI Advantage content" beschriftet. In TinyMCE (Standard ab Moodle 4.3) befindet sie sich im **Mehr**-Menü unter **Externe Tools**.
4. Wählen Sie **FastComments** aus der Tool-Liste.
5. FastComments öffnet einen Deep-Linking-Auswahldialog. Bestätigen Sie den Thread-Titel und klicken Sie auf **Embed**.
6. Der Editor fügt einen LTI-Platzhalterblock ein. Speichern Sie das Material.

Jede eingebettete Instanz ist ein eigener Thread, der an der Deep-Link-Content-Item-ID hängt, sodass eine Seite mit drei FastComments-Einbettungen drei unabhängige Threads erhält.

#### Zugriffsbeschränkung und Gruppeneinstellungen

Die Standard-Moodle-Aktivitätseinstellungen gelten auch für FastComments-Aktivitäten:

- **Allgemeine Moduleinstellungen** > **Gruppenmodus**. Die Einstellung **Getrennte Gruppen** oder **Sichtbare Gruppen** teilt FastComments nicht von selbst in gruppenbezogene Threads auf. Moodles Gruppenmodus filtert nur den Bewertungsbereich und die Mitgliederliste. Um einen separaten Thread pro Gruppe zu betreiben, fügen Sie eine FastComments-Aktivität pro Gruppe hinzu und verwenden Sie **Zugriff beschränken**, um jede einzelne einzugrenzen.
- **Zugriff beschränken** > **Beschränkung hinzufügen**. Unterstützt die Standard-Moodle-Bedingungen: **Datum**, **Bewertung**, **Gruppe**, **Gruppierung**, **Nutzerprofil** sowie verschachtelte Beschränkungssätze. Verwenden Sie **Gruppe**, um eine FastComments-Aktivität auf eine einzelne Gruppe zu beschränken.
- **Aktivitätsabschluss**. Setzen Sie auf **Studierende müssen diese Aktivität ansehen, um sie abzuschließen**, wenn Sie eine Abschlussverfolgung wünschen. FastComments meldet derzeit über den Start hinaus kein Abschlussereignis an Moodle zurück.

#### Rollenzuordnung

FastComments liest den LTI-`roles`-Claim, den Moodle bei jedem Start sendet, und ordnet ihn wie folgt zu:

- Moodle **Manager** oder **Site-Administrator** -> FastComments **admin**
- Moodle **Lehrkraft mit Bearbeitungsrecht** oder **Lehrkraft ohne Bearbeitungsrecht** -> FastComments **moderator**
- Moodle **Studierende/r** -> FastComments **commenter**
- Moodle **Gast** -> nur lesend

Admins können jeden Kommentar löschen, Nutzer sperren und Thread-Einstellungen bearbeiten. Moderatoren können Kommentare innerhalb des Threads löschen und freigeben, in den sie gestartet sind. Benutzerdefinierte Moodle-Rollen erben die Zuordnung des Archetyps, aus dem sie geklont wurden.

#### Was Studierende sehen

Studierende klicken auf die FastComments-Aktivität (oder scrollen zum eingebetteten Block innerhalb einer Seite oder eines Buchs). Moodle übermittelt ihre Identität über den LTI-Start an FastComments:

- Kein Anmeldebildschirm. FastComments meldet sie mit dem Moodle-Konto an.
- Anzeigename, E-Mail und Avatar stammen aus Moodle.
- Der Thread ist auf `(Moodle-Site, Kurs, Ressourcen-Link-ID)` beschränkt, sodass dieselbe Aktivität, die in einen anderen Kurs dupliziert wird, einen neuen Thread erhält.
- Verschachtelte Antworten, Abstimmungen und Benachrichtigungen funktionieren wie bei einem eigenständigen FastComments-Thread.

#### Öffentlichen Zugriff sperren (empfohlen)

Standardmäßig sind FastComments-Kommentardaten öffentlich lesbar. Jeder, der die URL eines Threads oder einen API-Endpunkt erraten kann, kann dessen Kommentare ansehen – auch außerhalb von Moodle. Für Kursdiskussionen möchten Sie die Ansicht fast sicher auf eingeschriebene Studierende beschränken.

Öffnen Sie Ihre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget-Anpassungsseite</a> und erstellen Sie eine Regel mit aktiviertem **Require SSO To View Comments**, und setzen Sie dann die Sicherheitsstufe auf **Secure SSO**, sodass Threads nur über den signierten LTI-Start geladen werden können.

Eine vollständige Anleitung, einschließlich der Eingrenzung der Regel auf eine einzelne Domain oder Seite, finden Sie unter [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments).

#### Moodle-Gotchas

**FastComments fehlt in der Aktivitätsauswahl.** Der Site-Administrator hat das Tool registriert, aber **Tool-Konfigurationsverwendung** nicht auf **In Aktivitätsauswahl und als vorkonfiguriertes Tool anzeigen** gesetzt. Beheben Sie dies unter **Website-Administration** > **Plugins** > **Aktivitätsmodule** > **Externes Tool** > **Tools verwalten** > Zahnrad-Symbol auf der FastComments-Kachel.

**Start schlägt fehl oder zeigt einen leeren Rahmen bei „Gleiches Fenster".** Moodles Session-Cookies verwenden standardmäßig `SameSite=Lax`, und manche Browser entfernen sie beim Cross-Site-POST, den LTI 1.3 für die Rückkehr von FastComments verwendet. Setzen Sie den **Start-Container** der Aktivität auf **Neues Fenster**. Dies ist eine zwingende Voraussetzung für eingebettete FastComments innerhalb einer Seite oder eines Buchs, da der editor-eingebettete Startpfad immer ein neues Fenster öffnet.

**Der `iss`-Claim ist die Moodle-Site-URL, keine Tenant-ID.** FastComments verwendet die Moodle-Site-URL (den `wwwroot`-Konfigurationswert) als LTI-Issuer. Wenn Ihre Moodle-Instanz auf eine neue Domain umzieht oder Sie `wwwroot` ändern, bleiben bestehende FastComments-Threads an den alten Issuer gebunden und passen nicht zu neuen Starts. Registrieren Sie das Tool unter der neuen URL erneut und migrieren Sie Threads bei Bedarf über die FastComments-Verwaltung.

**Aktivitätssicherung und -wiederherstellung.** Das Sichern eines Kurses und die Wiederherstellung in einem neuen Kurs erzeugt neue Ressourcen-Link-IDs, sodass die wiederhergestellten FastComments-Aktivitäten mit leeren Threads starten. Der ursprüngliche Kurs behält die ursprünglichen Threads. Dies ist beabsichtigtes Verhalten, kein Fehler.

**Moodle 4.5 TinyMCE-Standard.** Moodle 4.5 liefert TinyMCE als Standard-Editor für Neuinstallationen aus. Die Schaltfläche „Externes Tool" befindet sich im **Mehr**-Menü (`...`) statt in der Haupt-Symbolleiste. Ältere Sites, die von 4.1 aktualisiert wurden, behalten Atto, es sei denn, ein Administrator hat den Standard geändert.
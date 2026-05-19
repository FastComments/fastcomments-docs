Sobald ein Administrator FastComments als LTI 1.3 Advantage-Tool registriert und die Institutionseinstellungen genehmigt hat, fügen Lehrende es über die standardmäßigen Blackboard-Platzierungspunkte zu Kursen hinzu. Die genauen Schritte unterscheiden sich zwischen Ultra-Kursansicht und Original-Kursansicht, daher werden beide Varianten unten beschrieben.

#### Ultra-Kursansicht

Die Ultra-Kursansicht ist ab 2026 die Standardansicht in Blackboard Learn SaaS.

1. Öffnen Sie den Kurs und gehen Sie zur **Course Content**-Seite.
2. Fahren Sie mit der Maus über oder tippen Sie an die Stelle im Gliederungsbereich, an der der Kommentar-Thread erscheinen soll, und klicken Sie auf die violette **+**-Schaltfläche (Inhalt hinzufügen).
3. Wählen Sie **Content Market**. Das Content-Market-Panel listet alle genehmigten LTI-Tools und Building-Block-Platzierungen Ihrer Institution auf.
4. Finden Sie das **FastComments**-Kachel und klicken Sie darauf. Blackboard erstellt ein Inhaltelement an der Position, an der Sie das **+**-Menü geöffnet haben.
5. Standardmäßig erscheint das Element in der Gliederung als Eintrag "**Für Studierende sichtbar**" für Dozierende, die in ihren persönlichen Voreinstellungen **Vor Studierenden ausblenden** ausgeschaltet haben. Ist Ihre Voreinstellung **Ausgeblendet**, wird das Element verborgen erstellt und Sie schalten den Sichtbarkeitsregler in der Elementzeile um, wenn Sie bereit sind.
6. Um das Element umzubenennen, klicken Sie auf den Titel in der Gliederung und geben Sie ein neues Label ein. Der Titel, den Studierende in der Gliederung sehen, ist unabhängig von der FastComments-Thread-Kennung, daher ist ein Umbenennen jederzeit unproblematisch.

Wenn Sie **Content Market** nicht als Option sehen, hat Ihre Institution die Platzierung verborgen. Sie erreichen den gleichen Auswahlbildschirm auch über **More tools** im selben **+**-Menü unter der Gruppe **LTI Tools**.

#### Original-Kursansicht

Die Original-Kursansicht wird in Learn SaaS weiterhin unterstützt und bleibt die primäre Erfahrung für selbstgehostete Learn 9.1-Installationen auf der Q4-2024-CU-Release-Linie.

1. Öffnen Sie den Kurs und rufen Sie einen **Content Area** auf (zum Beispiel den Standardbereich **Information** oder **Content** im Kursmenü).
2. Schalten Sie den **Edit Mode** oben rechts auf der Seite ein.
3. Klicken Sie in der Aktionsleiste auf **Build Content**.
4. Klicken Sie im Untermenü **Learning Tools** auf **FastComments**. Das Untermenü Learning Tools wird nach der Registrierung des Tools mit LTI 1.3-Toolplatzierungen gefüllt. Wenn Sie es nicht sehen, lesen Sie den Abschnitt mit den Problemen (gotchas) weiter unten.
5. Auf dem Formular **Create FastComments** setzen Sie:
   - **Name**: die Beschriftung, die Studierende im Inhaltsbereich sehen.
   - **Description**: optionaler Text, der über dem eingebetteten Thread angezeigt wird.
   - **Permit Users to View this Content**: Ja/Nein-Verfügbarkeitsumschalter.
   - **Track Number of Views**: aktivieren, wenn Sie Blackboards Statistiken zur Anzahl der Aufrufe pro Element wünschen. FastComments führt eigene Analysen unabhängig davon aus.
   - **Date and Time Restrictions**: optionale **Display After**- / **Display Until**-Fenster.
6. Absenden. Das Tool erscheint als anklickbares Element im Inhaltsbereich.

#### Einbetten innerhalb eines Elements oder Dokuments

In beiden Kursansichten betten Lehrende FastComments inline in den Textkörper eines Elements, Dokuments oder eines beliebigen Rich-Text-Feldes über die LTI-Advantage-Schaltfläche des Inhaltseditors ein.

Ultra-Kursansicht:

1. Erstellen oder bearbeiten Sie ein **Document**.
2. Klicken Sie im Dokumentkörper an die Stelle, an der der Thread erscheinen soll, auf **Add content**.
3. Öffnen Sie in der Editor-Symbolleiste das Menü **Insert content** und klicken Sie auf **Content Market** (den Einstiegspunkt LTI Advantage / Deep Linking).
4. Wählen Sie **FastComments**. FastComments liefert eine Deep-Link-Payload und Blackboard fügt an der Cursorposition einen eingebetteten Block in den Dokumentkörper ein.
5. Speichern Sie das Dokument. Studierende sehen den Thread inline gerendert, wenn sie daran vorbeiscrollen.

Original-Kursansicht:

1. Bearbeiten Sie ein beliebiges Element mit Rich-Text-Körper.
2. Klicken Sie in der Symbolleiste des Inhaltseditors auf das Plus-Symbol **Add Content** und wählen Sie **Content Market** (in älteren Q4-2024-CUs als **Add Content from External Tool** bezeichnet).
3. Wählen Sie **FastComments**. Der Editor fügt einen Platzhalterblock ein, der auf die deep-gelinkte Ressource verweist.
4. Senden Sie das Element ab.

Jeder Deep-Link-Einbettung erzeugt einen eigenen FastComments-Thread, sodass ein Element mit zwei eingebetteten FastComments-Blöcken zwei unabhängige Kommentarströme hat.

#### Sichtbarkeit, Veröffentlichungsbedingungen und Gruppenbeschränkungen

FastComments-Inhaltelemente verhalten sich wie jedes andere Blackboard-Inhaltelement hinsichtlich der Zugriffskontrollregeln, die darauf angewendet werden.

- Ultra: Klicken Sie auf den Sichtbarkeitsselektor in der Zeile (**Für Studierende sichtbar**, **Für Studierende ausgeblendet**, **Bedingte Verfügbarkeit**). Die bedingte Verfügbarkeit unterstützt Datums-/Uhrzeitfenster, Leistungsregeln gegenüber Gradebook-Elementen und Mitgliedsregeln gegenüber Kursgruppen.
- Original: Öffnen Sie das Kontextmenü des Elements und wählen Sie **Adaptive Release** oder **Adaptive Release: Advanced**, um das Tool nach Datum, Mitgliedschaft, Note oder Prüfungsstatus zu sperren. Verwenden Sie **Set Group Availability** für das Element, um es auf bestimmte Kursgruppen zu beschränken.

FastComments respektiert, was auch immer Blackboards Sperre festlegt. Wenn Blackboard das Element für eine Studentin/einen Studenten ausblendet, findet für diese Person niemals der LTI-Start statt und sie erscheint nicht in der Moderatoransicht.

#### Verhalten im Gradebook

FastComments meldet keine Noten zurück über LTI Advantage Assignment and Grade Services. Es wird keine Notenspalte automatisch für FastComments-Inhaltelemente angelegt.

Wenn Ihr Blackboard-Mandant so konfiguriert ist, dass für jedes neue Inhaltelement unabhängig von Bewertungsmetadaten automatisch eine Notebuchspalte angelegt wird, erscheint trotzdem eine leere Spalte. Um sie auszublenden:

- Ultra: Öffnen Sie das **Gradebook**, klicken Sie auf den Spaltenkopf, wählen Sie **Edit** und schalten Sie **Show to students** sowie **Include in calculations** aus. Oder verwenden Sie **Delete**, falls Ihre Institution das Löschen von Spalten für nicht bewertete Elemente erlaubt.
- Original: Öffnen Sie das **Grade Center**, klicken Sie auf den Chevron der Spalte, wählen Sie **Hide from Users (on/off)** und optional **Hide from Instructor View** unter **Column Organization**.

#### Was Studierende sehen

Wenn eine Studentin bzw. ein Student das FastComments-Element öffnet oder zu einem eingebetteten Block scrollt:

1. Blackboard startet die LTI-1.3-Nachricht an FastComments. Die Studentin/der Student wird mittels SSO über ihre/seine Blackboard-Identität (Name, E-Mail, Avatar, Rolle) angemeldet, ohne ein Anmeldeformular zu sehen.
2. Der Kommentar-Thread wird im iframe gerendert. Threading, Antworten, Erwähnungen und Reaktionen sind je nach in FastComments konfigurierten Kommentar-Widget-Einstellungen verfügbar.
3. Ihre/seine Kommentare werden ihrem/seinem Blackboard-Konto zugeordnet. Wenn die Studentin/der Student später ihren/seinen Namen oder das Foto in Blackboard ändert, aktualisiert der nächste Start das FastComments-Profil.

Rollen-Mapping von Blackboard zu FastComments:

- **System Administrator** und **Course Builder** werden auf FastComments **admin** abgebildet.
- **Instructor** und **Teaching Assistant** werden auf FastComments **moderator** abgebildet.
- **Student**, **Guest** und **Observer** werden auf FastComments **commenter** abgebildet.

Moderatoren sehen Moderationskontrollen (pin, hide, ban, delete) inline an jedem Kommentar im Thread.

#### Öffentlich zugänglichen Zugriff einschränken (empfohlen)

Standardmäßig sind FastComments-Kommentardaten öffentlich lesbar. Jeder, der die URL oder den API-Endpunkt eines Threads errät, kann dessen Kommentare sehen, auch außerhalb von Blackboard. Für Kursdiskussionen möchten Sie in den allermeisten Fällen die Ansicht auf eingeschriebene Studierende beschränken.

Öffnen Sie Ihre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget-Anpassungsseite</a> und erstellen Sie eine Regel mit aktivierter Option **Require SSO To View Comments**, und setzen Sie dann das Sicherheitsniveau auf **Secure SSO**, damit Threads nur über den signierten LTI-Start geladen werden können.

Siehe [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) für die vollständige Anleitung, einschließlich wie Sie die Regel auf eine einzelne Domain oder Seite begrenzen.

#### Thread-Scoping

FastComments scoped jeden Thread nach **(Blackboard-Host, Kurs-ID, resource link ID)**. Zwei FastComments-Elemente im selben Kurs erzeugen zwei Threads. Dasselbe Element, das in zwei Kurs-Shells kopiert wird (zum Beispiel durch Kurskopie), erzeugt zwei Threads, weil Blackboard beim Kopiervorgang eine neue resource link ID ausgibt. Um einen gemeinsamen Thread über Kurskopien hinweg beizubehalten, verwenden Sie Deep Linking mit einer expliziten Thread-URN, die in FastComments vor dem Start der Kopie konfiguriert ist.

#### Blackboard-spezifische Probleme (Gotchas)

**FastComments-Kachel fehlt im Build Content-Menü (Original) oder Content Market (Ultra).** Der Administrator hat das Tool genehmigt, aber eine Institutionseinstellung hat die entsprechende Platzierung blockiert. Gehen Sie zu **Administrator Panel** > **Integrations** > **LTI Tool Providers**, bearbeiten Sie den FastComments-Eintrag und bestätigen Sie, dass sowohl **Course Content Tool** (Original) als auch **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) Platzierungen aktiviert sind. Speichern und aktualisieren Sie die Kursseite.

**Fehler "Tool not configured for this context" oder "Tool is not deployed" beim Start.** Der während der dynamischen Registrierung registrierte Deploymentscope stimmt nicht mit dem Institutionenkontext überein, zu dem der Kurs gehört. Prüfen Sie im Tool-Provider-Eintrag von Blackboard, ob die **Deployment ID** mit der übereinstimmt, die FastComments auf seiner LTI-1.3-Konfigurationsseite für diesen Mandanten anzeigt. Falls sie abweichen, löschen Sie die Platzierung und führen Sie die dynamische Registrierung erneut über eine frische Registrierungs-URL aus (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>).

**Iframe-Höhe wirkt fixiert oder Inhalte werden abgeschnitten.** Einige Blackboard-Mandanten verwenden eine strikte Content-Security-Policy, die das standardmäßige LTI-iframe-resize-postMessage blockiert. FastComments sendet sowohl die Canvas-typische `lti.frameResize`-Nachricht als auch die IMS-Spezifikations-Form `org.imsglobal.lti.frameResize`, um die Kompatibilität zu maximieren, aber eine mandantenweite CSP-Überschreibung kann den Parent-Listener blockieren. Bitten Sie Ihren Administrator zu bestätigen, dass `*.fastcomments.com` auf der LTI-Tool-Allowlist steht und dass kein benutzerdefinierter CSP-Header postMessage-Ereignisse entfernt. Danach funktioniert die Größenanpassung ohne weitere Konfiguration.

**Kurskopie dupliziert Threads.** Bei Kurskopien vergibt Blackboard neue resource link IDs für LTI-Platzierungen, sodass kopierte Kurse mit leeren Threads beginnen. Das ist zu erwarten. Wenn der kopierte Kurs den ursprünglichen Thread erben soll, richten Sie Deep Linking mit einer expliziten Thread-URN vor der Kopie ein, oder kontaktieren Sie den FastComments-Support, um Thread-IDs in großer Zahl neu zuzuordnen.

**Student/in sieht beim Start einen generischen Blackboard-Fehler.** Ursache ist ein fehlender oder veralteter `email`-Claim. Prüfen Sie die Institutionseinstellungen für FastComments und stellen Sie sicher, dass **Role**, **Name** und **Email Address** unter **User Fields to Send** aktiviert sind. Speichern Sie die Einstellungen und starten Sie erneut in einer frischen Browsersitzung.
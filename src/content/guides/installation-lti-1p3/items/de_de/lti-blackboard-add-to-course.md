Sobald ein Administrator FastComments als LTI 1.3 Advantage-Tool registriert und die institutionellen Richtlinien genehmigt hat, fügen Lehrende es über die standardmäßigen Blackboard-Platzierungsstellen zu Kursen hinzu. Die genauen Schritte unterscheiden sich zwischen Ultra Course View und Original Course View, daher werden beide unten beschrieben.

#### Ultra Course View

Ultra Course View ist die Standardansicht in Blackboard Learn SaaS seit 2026.

1. Öffnen Sie den Kurs und gehen Sie zur **Course Content**-Seite.
2. Bewegen Sie den Mauszeiger oder tippen Sie an die Stelle im Gliederungsbaum, an der der Kommentar-Thread erscheinen soll, und klicken Sie auf die lila **+** (Add content)-Schaltfläche.
3. Wählen Sie **Content Market**. Das Content Market-Panel listet alle genehmigten LTI-Tools und Building Block-Platzierungen für Ihre Institution auf.
4. Finden Sie die **FastComments**-Kachel und klicken Sie darauf. Blackboard erstellt ein Inhaltselement an der Position, an der Sie das **+**-Menü geöffnet haben.
5. Das Element erscheint standardmäßig als Eintrag „Visible to students“, sofern Lehrende in ihren persönlichen Einstellungen **Hide from students** nicht aktiviert haben. Ist Ihre Voreinstellung **Hidden**, wird das Element versteckt erstellt und Sie schalten den Sichtbarkeitsregler in der Elementzeile um, wenn Sie bereit sind.
6. Um das Element umzubenennen, klicken Sie auf den Titel in der Gliederung und geben Sie ein neues Label ein. Der Titel, den Studierende in der Gliederung sehen, ist unabhängig von der FastComments-Thread-Kennung, sodass ein Umbenennen jederzeit sicher ist.

Wenn Sie **Content Market** nicht als Option sehen, hat Ihre Institution die Platzierung verborgen. Sie erreichen denselben Auswähler auch über **More tools** im selben **+**-Menü unter der Gruppe **LTI Tools**.

#### Original Course View

Original Course View wird in Learn SaaS weiterhin unterstützt und bleibt die primäre Erfahrung für selbstgehostete Learn 9.1-Sites auf der Q4 2024 CU-Release-Linie.

1. Öffnen Sie den Kurs und betreten Sie einen **Content Area** (zum Beispiel den standardmäßigen Bereich **Information** oder **Content** im Kursmenü).
2. Schalten Sie oben rechts auf der Seite den **Edit Mode**-Schalter ein.
3. Klicken Sie in der Aktionsleiste auf **Build Content**.
4. Unter dem Untermenü **Learning Tools** klicken Sie auf **FastComments**. Das Untermenü Learning Tools wird nach der Registrierung des Tools durch einen Administrator mit LTI 1.3-Toolplatzierungen gefüllt. Wenn Sie es nicht sehen, siehe den Abschnitt „gotchas“ unten.
5. Auf dem Formular **Create FastComments** legen Sie fest:
   - **Name**: das Label, das Studierende im Inhaltsbereich sehen.
   - **Description**: optionaler Text, der über dem eingebetteten Thread angezeigt wird.
   - **Permit Users to View this Content**: Ja/Nein-Verfügbarkeitsumschalter.
   - **Track Number of Views**: aktivieren, wenn Sie die per-Item-Ansichtsstatistiken von Blackboard nutzen möchten. FastComments führt eigene Analytics unabhängig davon aus.
   - **Date and Time Restrictions**: optionale **Display After** / **Display Until**-Fenster.
6. Absenden. Das Tool erscheint als klickbares Element im Inhaltsbereich.

#### Embedding Inside an Item or Document

In beiden Kursansichten betten Lehrende FastComments inline in den Textkörper eines Items, Dokuments oder eines beliebigen Rich-Text-Felds über die LTI Advantage-Schaltfläche des Content Editors ein.

Ultra Course View:

1. Erstellen oder bearbeiten Sie ein **Document**.
2. Klicken Sie im Dokumentkörper auf **Add content** an der Stelle, an der der Thread erscheinen soll.
3. Öffnen Sie in der Editor-Symbolleiste das Menü **Insert content** und klicken Sie auf **Content Market** (den Einstiegspunkt LTI Advantage / Deep Linking).
4. Wählen Sie **FastComments**. FastComments liefert eine Deep-Link-Payload und Blackboard fügt an der Cursor-Position einen eingebetteten Block in den Dokumentkörper ein.
5. Speichern Sie das Dokument. Studierende sehen den Thread inline gerendert, wenn sie daran vorbeiscrollen.

Original Course View:

1. Bearbeiten Sie ein beliebiges Item mit einem Rich-Text-Body.
2. Klicken Sie in der Symbolleiste des Content Editors auf das Plus-Symbol **Add Content** und wählen Sie **Content Market** (in älteren Q4 2024 CUs als **Add Content from External Tool** beschriftet).
3. Wählen Sie **FastComments**. Der Editor fügt einen Platzhalterblock ein, der auf die Deep-Linked-Ressource verweist.
4. Senden Sie das Item ab.

Jede Deep-Link-Einbettung erzeugt ihren eigenen FastComments-Thread, sodass ein Item mit zwei eingebetteten FastComments-Blöcken zwei unabhängige Kommentarströme hat.

#### Visibility, Release Conditions, and Group Restrictions

FastComments-Inhaltselemente verhalten sich wie andere Blackboard-Inhaltselemente hinsichtlich der darauf angewendeten Zugriffskontrollregeln.

- Ultra: Klicken Sie auf den Sichtbarkeitsregler in der Zeile (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability unterstützt Datums-/Uhrzeitfenster, Performance-Regeln gegen Gradebook-Items und Mitgliedsregeln gegen Kursgruppen.
- Original: Öffnen Sie das Kontextmenü des Elements und wählen Sie **Adaptive Release** oder **Adaptive Release: Advanced**, um das Tool nach Datum, Mitgliedschaft, Note oder Prüfungsstatus zu sperren. Verwenden Sie **Set Group Availability** am Element, um es auf bestimmte Kursgruppen zu beschränken.

FastComments respektiert, was auch immer das Blackboard-Gate entscheidet. Wenn Blackboard das Element für eine*n Studierende*n verbirgt, findet für diese Person kein LTI-Launch statt und sie erscheint nicht in der Moderatoransicht.

#### Gradebook Behavior

FastComments meldet keine Noten über LTI Advantage Assignment and Grade Services zurück. Es wird keine Notenspalte automatisch für FastComments-Inhaltselemente erstellt.

Wenn Ihr Blackboard-Tenant so konfiguriert ist, dass für jedes neue Inhaltselement unabhängig von Bewertungsmetadaten automatisch eine Notenspalte erstellt wird, erscheint trotzdem eine leere Spalte. Um sie auszublenden:

- Ultra: Öffnen Sie das **Gradebook**, klicken Sie auf den Spaltenkopf, wählen Sie **Edit** und deaktivieren Sie **Show to students** sowie **Include in calculations**. Oder verwenden Sie **Delete**, wenn Ihre Institution das Löschen von Spalten für unbewertete Elemente erlaubt.
- Original: Öffnen Sie das **Grade Center**, klicken Sie auf den Chevron der Spalte, wählen Sie **Hide from Users (on/off)** und optional **Hide from Instructor View** unter **Column Organization**.

#### What Students See

Wenn eine Studentin oder ein Student das FastComments-Element öffnet oder zu einem eingebetteten Block scrollt:

1. Blackboard startet die LTI 1.3-Nachricht an FastComments. Die Studentin/der Student ist per SSO mit ihrer/seiner Blackboard-Identität (Name, E-Mail, Avatar, Rolle) angemeldet, ohne ein Anmeldeformular zu sehen.
2. Der Kommentar-Thread rendert im iframe. Threading, Antworten, Erwähnungen und Reaktionen sind je nach in FastComments konfigurierten Kommentar-Widget-Einstellungen verfügbar.
3. Ihre/seine Kommentare werden ihrem/seinem Blackboard-Konto zugeordnet. Wenn die Studentin/der Student später ihren/seinen Namen oder ihr/sein Foto in Blackboard ändert, aktualisiert der nächste Launch das FastComments-Profil.

Role-Mapping von Blackboard zu FastComments:

- **System Administrator** und **Course Builder** map to FastComments **admin**.
- **Instructor** und **Teaching Assistant** map to FastComments **moderator**.
- **Student**, **Guest**, und **Observer** map to FastComments **commenter**.

Moderators sehen Moderationskontrollen (pin, hide, ban, delete) inline an jedem Kommentar im Thread.

#### Thread Scoping

FastComments scopet jeden Thread nach **(Blackboard host, course ID, resource link ID)**. Zwei FastComments-Elemente im selben Kurs erzeugen zwei Threads. Dasselbe Element, das in zwei Kurs-Instanzen kopiert wird (zum Beispiel durch Course Copy), erzeugt zwei Threads, da Blackboard beim Kopieren eine neue resource link ID ausstellt. Um einen geteilten Thread über Kurskopien hinweg beizubehalten, verwenden Sie Deep Linking mit einer expliziten Thread-URN, die in FastComments vor dem Starten der Kopie konfiguriert wird.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** Der Administrator hat das Tool genehmigt, aber eine institutionelle Richtlinie die relevante Platzierung blockiert. Gehen Sie zu **Administrator Panel** > **Integrations** > **LTI Tool Providers**, bearbeiten Sie den FastComments-Eintrag und prüfen Sie, dass sowohl **Course Content Tool** (Original) als auch **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) Platzierungen aktiviert sind. Speichern und aktualisieren Sie die Kursseite.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** Der während der dynamischen Registrierung registrierte Deployment-Scope stimmt nicht mit dem Institutionskontext überein, zu dem der Kurs gehört. Überprüfen Sie im Tool-Provider-Eintrag von Blackboard, ob die **Deployment ID** mit der übereinstimmt, die FastComments auf seiner LTI 1.3-Configuration-Seite für diesen Tenant anzeigt. Falls sie abweichen, löschen Sie die Platzierung und führen Sie die dynamische Registrierung erneut über eine frische Registrierungs-URL aus (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>).

**Iframe height looks fixed or content gets cut off.** Manche Blackboard-Tenants liefern eine strikte Content Security Policy aus, die die standardmäßige LTI-iframe-resize postMessage blockiert. FastComments sendet sowohl die Canvas-Style-`lti.frameResize`-Nachricht als auch die IMS-Spec-Form `org.imsglobal.lti.frameResize`-Nachricht, um die Kompatibilität zu maximieren, aber eine Tenant-weite CSP-Überschreibung blockiert den Parent-Listener. Bitten Sie Ihren Administrator zu bestätigen, dass `*.fastcomments.com` auf der LTI-Tool-Allowlist steht und dass kein benutzerdefinierter CSP-Header postMessage-Events entfernt. Danach funktioniert das Resizing ohne weitere Konfiguration.

**Course copy duplicates threads.** Beim Blackboard Course Copy werden neue resource link IDs für LTI-Platzierungen ausgegeben, sodass kopierte Kurse mit leeren Threads starten. Das ist erwartetes Verhalten. Wenn der kopierte Kurs den ursprünglichen Thread erben soll, richten Sie Deep Linking mit einer expliziten Thread-URN vor dem Kopieren ein oder kontaktieren Sie den FastComments-Support, um Thread-IDs in großer Zahl neu zuzuordnen.

**Student sees a generic Blackboard error on launch.** Ursache ist ein fehlender oder veralteter `email`-Claim. Stellen Sie sicher, dass in der Institution Policy für FastComments unter **User Fields to Send** die Felder **Role**, **Name** und **Email Address** aktiviert sind. Speichern Sie und starten Sie dann in einer frischen Browsersitzung erneut.
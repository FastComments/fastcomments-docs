Sobald FastComments in die Plattform registriert ist, fügen Lehrende es den Kursinhalten über die standardmäßigen Abläufe für externe Tools der Plattform hinzu. Diese Seite behandelt Sakai 23.x und Schoology Enterprise.

#### Sakai

**1. FastComments zu einer Site hinzufügen**

Der Site‑Maintainer aktiviert das Tool pro Site:

1. Öffnen Sie die Site und klicken Sie in der linken Navigation auf **Site Info**.
2. Klicken Sie auf **Manage Tools**.
3. Scrollen Sie zur Liste **External Tools** und schalten Sie **FastComments** ein.
4. Klicken Sie auf **Continue**, prüfen Sie die Tool‑Liste und klicken Sie dann auf **Finish**.

FastComments erscheint jetzt als Eintrag in der linken Navigation der Site.

**2. Die Position des Eintrags in der linken Navigation ändern**

Gehen Sie zu **Site Info** > **Tool Order**. Ziehen Sie **FastComments** an die gewünschte Position und klicken Sie auf **Save**. Über diesen Bildschirm können Sie das Navigationslabel auch umbenennen und es für Studierende ausblenden.

**3. Inline in einer Lessons‑Seite einbetten**

Um FastComments direkt innerhalb einer Lessons‑Seite und nicht als eigenständiges Tool in der linken Navigation zu platzieren:

1. Öffnen Sie das Tool **Lessons** in der Site.
2. Klicken Sie auf **Add Content** > **Add External Tool**.
3. Wählen Sie **FastComments** aus der Liste.
4. Wenn FastComments während der Registrierung Deep Linking angekündigt hat, öffnet Sakai den Inhaltsselektor des Tools, sodass Sie den Thread auswählen oder beschriften können. Wenn Deep Linking nicht angekündigt wurde, fügt Sakai einen Standard‑Startlink ein.
5. Speichern Sie das Lessons‑Element.

Jede eingebettete Instanz erhält ihren eigenen Thread, der auf diesen Ressourcenlink beschränkt ist.

**4. Zugriffsberechtigungen für Studierende anpassen**

Sakai steuert externe Tool‑Starts über Realms. Um zu bestätigen, dass Studierende FastComments starten können:

1. Melden Sie sich als Sakai‑Admin an und öffnen Sie **Administration Workspace** > **Realms**.
2. Öffnen Sie den relevanten Realm (zum Beispiel `!site.template.course` oder den spezifischen Site‑Realm).
3. Bestätigen Sie, dass die Rolle `access` `lti.launch` aktiviert hat und dass die Rollenberechtigungen in der Gruppe **external.tools** gewährt sind.
4. Speichern Sie den Realm.

Für Site‑weite Überschreibungen kann der Maintainer die Tool‑Sichtbarkeit pro Rolle unter **Site Info** > **Tool Order** anpassen, indem FastComments pro Rolle ausgeblendet oder angezeigt wird.

**5. Was Studierende sehen**

Studierende klicken auf den FastComments‑Eintrag in der linken Navigation (oder scrollen zum eingebetteten Lessons‑Block) und landen direkt in der Thread‑Ansicht der Kommentare. SSO ist automatisch: Sakai sendet die Identität des Benutzers im LTI launch und FastComments meldet sie unter ihrem Sakai‑Konto an.

Rollen‑Mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai Fallstricke**

- **Tool nicht in Manage Tools sichtbar.** Wenn FastComments nicht in der Liste External Tools erscheint, muss der Sakai‑Admin das Tool‑Register öffnen (**Administration Workspace** > **External Tools** > **FastComments**) und **Stealthed** auf `false` setzen. Stealthed‑Tools sind im per‑Site Manage Tools‑Auswahlfeld ausgeblendet.
- **Starts brechen in Browsern mit geteilten Sitzungen.** Sakais Portal‑CSRF‑Token ist an die Browsersitzung gebunden. Wenn ein Studierender in zwei Sakai‑Sites in verschiedenen Tabs angemeldet ist oder eine veraltete Sitzung hat, liefert der Start einen 403. Lösung: Schließen Sie andere Sakai‑Tabs, melden Sie sich ab, melden Sie sich wieder an und starten Sie neu. Admins können auch `sakai.csrf.token.cache.ttl` erhöhen, wenn dieses Problem clusterweit auftritt.
- **Frame‑Einbettung.** Stellen Sie sicher, dass `lti.frameheight` in `sakai.properties` groß genug ist (600 oder höher), damit der Kommentarthread auf einer Lessons‑Seite nicht abgeschnitten wird.

#### Schoology

Schoology Enterprise hat zwei Installationsszenarien. Bestätigen Sie, welches zutrifft, bevor Sie das Tool zu einem Kurs hinzufügen.

**1. Zwei Installationsszenarien**

- **(a) Installation auf Unternehmensebene.** Der Schoology System Administrator hat FastComments auf Organisationsebene installiert und es allen Kursen oder bestimmten Kurvorlagen zugewiesen. Lehrende überspringen die Installation und gehen direkt zu „Add Materials“.
- **(b) Selbstinstallation durch Lehrende.** Die Lehrkraft installiert das Tool in einen einzelnen Kurs über **Course Options** > **External Tools** > **Install LTI Apps**. Die Selbstinstallation erfordert, dass der System Administrator die FastComments‑App zuvor auf Org‑Ebene genehmigt hat.

**2. FastComments als Kursmaterial hinzufügen**

Innerhalb des Kurses:

1. Öffnen Sie den Kurs und gehen Sie zu **Materials**.
2. Klicken Sie auf **Add Materials** > **Add File/Link/External Tool**.
3. Wählen Sie **External Tool**.
4. Wählen Sie **FastComments** aus der Liste der registrierten Tools.
5. Legen Sie einen **Name** fest (dies sehen Studierende in der Materialliste) und optional eine **Description**.
6. Lassen Sie **Enable Grading** (grade passback) **OFF**. FastComments meldet keine Noten an Schoology zurück; das Aktivieren des Grade‑Passback erzeugt eine leere Spalte im Notenbuch.
7. Klicken Sie auf **Submit**.

Das Material erscheint nun in der Kursmaterialliste und öffnet den FastComments‑Thread beim Klicken.

**3. Inline‑Einbettung über den Rich‑Text‑Editor**

Wenn der System Administrator während der Registrierung die Deep‑Linking‑Platzierung für FastComments aktiviert hat, können Lehrende den Kommentarthread in jedes Rich‑Text‑Feld einbetten (Anweisungen zu Aufgaben, Seiteninhalte, Diskussionsaufforderungen):

1. Öffnen Sie den Rich‑Text‑Editor auf der Zielseite.
2. Klicken Sie auf das **External Tool** (Puzzle‑Teil) Symbol in der Toolbar.
3. Wählen Sie **FastComments**.
4. Konfigurieren Sie die Einbettung im Deep‑Linking‑Dialog und klicken Sie auf **Insert**.
5. Speichern Sie die Seite.

Wenn die Schaltfläche External Tool im Rich‑Text‑Editor nicht erscheint, ist Deep Linking für dieses Tool in diesem Mandanten deaktiviert. Siehe die Fallstricke unten.

**4. Sichtbarkeit und Abschnittszuweisungen**

Schoology steuert die Tool‑Verfügbarkeit pro Abschnitt über Course Options:

1. Klicken Sie im Kurs auf **Course Options** > **External Tools**.
2. Für jede installierte LTI‑App steuern Sie, ob sie für alle Abschnitte im Kurs oder nur für bestimmte Abschnitte verfügbar ist.
3. Um FastComments auf bestimmte Abschnitte zu beschränken, deaktivieren Sie die Abschnitte, die das Tool nicht sehen sollen.
4. Der Abschnitts‑level Zugriff steuert auch, welche Abschnitte den Eintrag **Add Materials** > **External Tool** für FastComments sehen.

**5. Was Studierende sehen**

Studierende klicken auf das FastComments‑Material (oder scrollen zur Inline‑Einbettung) und landen in der Thread‑Diskussion. SSO ist automatisch über den Schoology LTI launch unter ihrem Schoology‑Konto.

Rollen‑Mapping:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology Fallstricke**

- **Nur Enterprise.** Persönliche und kostenlose Schoology‑Konten können keine LTI 1.3‑Tools installieren. Wenn Ihr Mandant den kostenlosen Tarif nutzt, fehlt die Option **External Tools** in Course Options. Upgraden Sie auf Schoology Enterprise, um FastComments zu verwenden.
- **Deep Linking standardmäßig vom Mandanten deaktiviert.** Manche Schoology‑Mandanten schränken die Deep‑Linking‑Platzierung auf Organisations‑Ebene ein. In diesem Fall sehen Lehrende nur den Flow **Add Materials** > **External Tool** und nicht die External Tool‑Schaltfläche im Rich‑Text‑Editor. Um die Inline‑Einbettung zu ermöglichen, geht der System Administrator zu **System Settings** > **Integration** > **LTI 1.3** > **FastComments**, aktiviert die Platzierung **Content Item / Deep Linking** und speichert.
- **Abschnittszuweisungs‑Überschreibung.** Wenn FastComments auf Unternehmensebene zugewiesen ist, die Lehrkraft es in **Add Materials** aber nicht sieht, ist der Kursabschnitt in der Org‑Zuweisung ausgeschlossen. Bitten Sie den System Administrator, den Abschnitt der FastComments‑App‑Zuweisung hinzuzufügen.
- **Materialname vs. Thread‑Identität.** Das Umbenennen des Materials in Schoology verschiebt den Kommentarthread nicht. Threads sind am LTI‑Resource‑Link‑ID‑Key gebunden, daher bleibt bei einer Umbenennung derselbe Thread erhalten; das Löschen und Neuerstellen des Materials erzeugt einen neuen, leeren Thread.
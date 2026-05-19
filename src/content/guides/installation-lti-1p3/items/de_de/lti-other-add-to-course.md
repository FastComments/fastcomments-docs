Sobald FastComments in der Plattform registriert ist, fügen Lehrende es mithilfe der standardmäßigen Abläufe für externe Tools der Plattform zu Kursinhalten hinzu. Diese Seite behandelt Sakai 23.x und Schoology Enterprise.

#### Öffentlichen Zugriff sperren (empfohlen)

Standardmäßig sind FastComments-Kommentardaten auf beiden Plattformen öffentlich lesbar. Jede Person, die die URL eines Threads oder einen API-Endpunkt errät, kann dessen Kommentare ansehen, auch außerhalb von Sakai oder Schoology. Für Kursdiskussionen möchten Sie die Ansicht in der Regel nur auf eingeschriebene Studierende beschränken.

Öffnen Sie Ihre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget-Anpassungsseite</a> und erstellen Sie eine Regel mit **Require SSO To View Comments** aktiviert, und setzen Sie dann das Sicherheitsniveau auf **Secure SSO**, sodass Threads nur über den signierten LTI-Start geladen werden können.

Siehe [Kommentarthreads mit Single-Sign-On schützen](/guide-customizations-and-configuration.html#sso-require-to-view-comments) für die vollständige Schritt-für-Schritt-Anleitung, einschließlich wie Sie die Regel auf eine einzelne Domain oder Seite eingrenzen.

#### Sakai

**1. FastComments zu einer Site hinzufügen**

Der Site-Maintainer aktiviert das Tool pro Site:

1. Öffnen Sie die Site und klicken Sie in der linken Navigation auf **Site Info**.
2. Klicken Sie auf **Manage Tools**.
3. Scrollen Sie zur Liste **External Tools** und schalten Sie **FastComments** ein.
4. Klicken Sie auf **Continue**, prüfen Sie die Tool-Liste und klicken Sie dann auf **Finish**.

FastComments erscheint nun als Left-Nav-Eintrag in der Site.

**2. Den Left-Nav-Eintrag neu anordnen**

Gehen Sie zu **Site Info** > **Tool Order**. Ziehen Sie **FastComments** an die gewünschte Position und klicken Sie auf **Save**. Auf diesem Bildschirm können Sie auch die Navigationsbezeichnung umbenennen und sie für Studierende ausblenden.

**3. Inline in einer Lessons-Seite einbetten**

Um FastComments direkt in eine Lessons-Seite einzufügen, anstatt es als eigenständiges Left-Nav-Tool zu verwenden:

1. Öffnen Sie das Tool **Lessons** in der Site.
2. Klicken Sie auf **Add Content** > **Add External Tool**.
3. Wählen Sie **FastComments** aus der Liste.
4. Falls FastComments beim Registrieren Deep Linking beworben hat, öffnet Sakai den Content-Selector des Tools, sodass Sie den Thread auswählen oder beschriften können. Wenn Deep Linking nicht beworben wurde, fügt Sakai einen Standard-Startlink ein.
5. Speichern Sie das Lessons-Element.

Jede eingebettete Instanz erhält ihren eigenen Thread, der auf diesen Resource Link begrenzt ist.

**4. Berechtigungsanpassungen für Studierendenzugang**

Sakai steuert externe Tool-Starts über Realms. Um zu bestätigen, dass Studierende FastComments starten können:

1. Melden Sie sich als Sakai-Admin an und öffnen Sie **Administration Workspace** > **Realms**.
2. Öffnen Sie das entsprechende Realm (zum Beispiel `!site.template.course` oder das spezifische Site-Realm).
3. Bestätigen Sie, dass die Rolle `access` `lti.launch` aktiviert hat und dass die Rollenberechtigungen in der **external.tools**-Gruppe gewährt sind.
4. Speichern Sie das Realm.

Für Site-weit geltende Überschreibungen kann der Maintainer die Tool-Sichtbarkeit pro Rolle über **Site Info** > **Tool Order** anpassen, indem FastComments für Rollen ein- oder ausgeblendet wird.

**5. Was Studierende sehen**

Studierende klicken auf den FastComments-Left-Nav-Eintrag (oder scrollen zum eingebetteten Lessons-Block) und landen direkt in der Thread-Ansicht der Kommentare. SSO ist automatisch: Sakai sendet die Identität des Nutzers im LTI-Start und FastComments meldet sie unter ihrem Sakai-Konto an.

Rollen-Mapping:

- Sakai `Instructor` -> FastComments-Moderator
- Sakai `Admin` (Admin im Administration Workspace) -> FastComments-Administrator
- Sakai `Student` / `access` -> FastComments-Kommentator

**6. Sakai Fallstricke**

- **Tool nicht in Manage Tools sichtbar.** Falls FastComments nicht in der Liste External Tools erscheint, muss der Sakai-Admin das Tool-Registry öffnen (**Administration Workspace** > **External Tools** > **FastComments**) und **Stealthed** auf `false` setzen. Stealthed-Tools sind im per-Site Manage Tools-Picker verborgen.
- **Starts brechen in Browsern mit geteilten Sessions.** Das Portal-CSRF-Token von Sakai ist an die Browsersession gebunden. Wenn ein Studierender in zwei Sakai-Sites in unterschiedlichen Tabs angemeldet ist oder eine veraltete Session hat, liefert der Start einen 403-Fehler. Lösung: Schließen Sie andere Sakai-Tabs, melden Sie sich ab, melden Sie sich wieder an und starten Sie neu. Admins können außerdem `sakai.csrf.token.cache.ttl` erhöhen, falls dies clusterweit passiert.
- **Frame-Einbettung.** Stellen Sie sicher, dass `lti.frameheight` in `sakai.properties` groß genug ist (600 oder höher), damit der Kommentar-Thread auf einer Lessons-Seite nicht abgeschnitten wird.

#### Schoology

Schoology Enterprise hat zwei Installationsszenarien. Bestätigen Sie, welches zutrifft, bevor Sie das Tool zu einem Kurs hinzufügen.

**1. Zwei Installationsszenarien**

- **(a) Install auf Enterprise-Ebene.** Der Schoology-Systemadministrator hat FastComments auf Organisationsebene installiert und es allen Kursen oder bestimmten Kurvorlagen zugewiesen. Lehrende überspringen die Installation und gehen direkt zu "Add Materials".
- **(b) Selbstinstallation durch Lehrende.** Die Lehrkraft installiert das Tool in einen einzelnen Kurs über **Course Options** > **External Tools** > **Install LTI Apps**. Die Selbstinstallation erfordert, dass der Systemadministrator die FastComments-App auf Organisationsebene zuvor genehmigt hat.

**2. FastComments als Kursmaterial hinzufügen**

Innerhalb des Kurses:

1. Öffnen Sie den Kurs und gehen Sie zu **Materials**.
2. Klicken Sie auf **Add Materials** > **Add File/Link/External Tool**.
3. Wählen Sie **External Tool**.
4. Wählen Sie **FastComments** aus der Liste der registrierten Tools.
5. Geben Sie einen **Name** an (dies sehen Studierende in der Materials-Liste) und eine optionale **Description**.
6. Lassen Sie **Enable Grading** (Grade Passback) **OFF**. FastComments meldet keine Noten an Schoology zurück; das Aktivieren der Notenübertragung erzeugt eine leere Spalte im Notenbuch.
7. Klicken Sie auf **Submit**.

Das Material erscheint nun in der Materials-Liste des Kurses und öffnet den FastComments-Thread beim Anklicken.

**3. Inline-Einbettung über den Rich Text-Editor**

Wenn der Systemadministrator während der Registrierung Deep Linking für FastComments aktiviert hat, können Lehrende den Kommentar-Thread in jedes Rich-Text-Feld einbetten (Anweisungen zu Aufgaben, Seiteninhalte, Diskussionsaufforderungen):

1. Öffnen Sie den Rich Text-Editor auf der Zielseite.
2. Klicken Sie auf das Symbol **External Tool** (Puzzleteil) in der Symbolleiste.
3. Wählen Sie **FastComments**.
4. Konfigurieren Sie die Einbettung im Deep-Linking-Dialog und klicken Sie auf **Insert**.
5. Speichern Sie die Seite.

Wenn die Schaltfläche External Tool im Rich Text-Editor nicht erscheint, ist Deep Linking für dieses Tool auf diesem Mandanten deaktiviert. Siehe die untenstehenden Fallstricke.

**4. Sichtbarkeit und Abschnittszuweisungen**

Schoology regelt die Tool-Verfügbarkeit pro Abschnitt über Course Options:

1. Klicken Sie aus dem Kurs heraus auf **Course Options** > **External Tools**.
2. Für jede installierte LTI-App steuern Sie, ob sie allen Abschnitten im Kurs oder nur bestimmten Abschnitten zur Verfügung steht.
3. Um FastComments auf bestimmte Abschnitte zu beschränken, deaktivieren Sie die Abschnitte, die das Tool nicht sehen sollen.
4. Der Abschnittsbezogene Zugriff steuert auch, welche Abschnitte den Eintrag **Add Materials** > **External Tool** für FastComments sehen.

**5. Was Studierende sehen**

Studierende klicken das FastComments-Material an (oder scrollen zur Inline-Einbettung) und landen in der Thread-Diskussion. SSO erfolgt automatisch über den Schoology-LTI-Start unter ihrem Schoology-Konto.

Rollen-Mapping:

- Schoology `Administrator` -> FastComments-Administrator
- Schoology `Instructor` -> FastComments-Moderator
- Schoology `Student` -> FastComments-Kommentator

**6. Schoology Fallstricke**

- **Nur Enterprise.** Persönliche und kostenlose Schoology-Konten können keine LTI 1.3-Tools installieren. Wenn Ihr Mandant die Free-Stufe nutzt, fehlt die Option **External Tools** in Course Options. Upgraden Sie auf Schoology Enterprise, um FastComments zu verwenden.
- **Deep Linking standardmäßig vom Mandanten deaktiviert.** Manche Schoology-Mandanten schränken die Deep-Linking-Platzierung auf Organisationsebene ein. In diesem Fall sehen Lehrende nur den Flow **Add Materials** > **External Tool** und nicht die External Tool-Schaltfläche im Rich Text-Editor. Um die Inline-Einbettung zu ermöglichen, geht der Systemadministrator zu **System Settings** > **Integration** > **LTI 1.3** > **FastComments** und aktiviert die Platzierung **Content Item / Deep Linking**, dann speichert er.
- **Abschnittsübersteuerung.** Wenn FastComments auf Enterprise-Ebene zugewiesen ist, die Lehrkraft es aber nicht unter **Add Materials** sieht, ist der Abschnitt im org-weiten Assignment ausgeschlossen. Bitten Sie den Systemadministrator, den Abschnitt zur FastComments-App-Zuweisung hinzuzufügen.
- **Materialname vs. Thread-Identität.** Das Umbenennen des Materials in Schoology verschiebt den Kommentar-Thread nicht. Threads sind am LTI-Resource-Link-ID verknüpft; ein Umbenennen belässt den Thread, das Löschen und Neuerstellen des Materials erzeugt einen neuen, leeren Thread.
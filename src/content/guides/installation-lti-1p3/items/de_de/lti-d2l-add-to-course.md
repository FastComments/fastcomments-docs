Diese Seite behandelt das Hinzufügen von FastComments zu einem Brightspace-Kurs, nachdem ein Administrator das Tool registriert und eine Bereitstellung erstellt hat. Falls das Tool noch nicht registriert ist, sehen Sie zuerst die D2L-Registrierungsanleitung.

<div class="screenshot white-bg">
    <div class="title">FastComments eingebettet als Unit-Thema in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments läuft innerhalb einer Brightspace-Unit und zeigt Threaded-Kommentare sowie einen @-Mention-Auswähler" />
</div>

Brightspace bietet zwei Content-Authoring-Erlebnisse: **Classic Content** und die **New Content Experience** (auch **Lessons** genannt). Beide zeigen FastComments an, aber die Menüpfade unterscheiden sich. Jeder nachfolgende Abschnitt behandelt beide, wo sie auseinandergehen.

#### FastComments-Tool finden

Das FastComments-Tool erscheint an zwei Stellen im Content-Editor eines Kurses:

1. Im Aktivitätsauswahl-Dialog, erreichbar über die **Add Existing**-Schaltfläche eines Moduls/Units (in älteren Brightspace-Versionen als **Add Existing Activities** beschriftet). In aktuellen Brightspace-Versionen erscheint FastComments direkt im Picker; ältere Versionen verschachteln es unter einem Untermenü **External Learning Tools**. Jeder Pfad fügt FastComments als eigenständiges Thema hinzu.
2. Im **Insert Stuff**-Dialog innerhalb des HTML-Editors, unter **LTI Advantage**. Dies bettet FastComments inline in ein HTML-Thema über den LTI-Deep-Linking-Flow ein.

Wenn FastComments in keinem der beiden Picker erscheint, ist die Bereitstellung für die organisatorische Einheit, die den Kurs enthält, nicht aktiviert. Bitten Sie Ihren Brightspace-Administrator, **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** zu öffnen, die Bereitstellung zu öffnen und die Organisations-Einheit des Kurses (oder eine übergeordnete Einheit) unter **Org Units** hinzuzufügen.

#### FastComments als Thema in einem Modul hinzufügen

Classic Content:

1. Öffnen Sie den Kurs und klicken Sie in der Navigationsleiste auf **Content**.
2. Wählen Sie das Modul aus, das die Diskussion enthalten soll (oder erstellen Sie eines über **Add a module**).
3. Klicken Sie auf **Add Existing** (älteres Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klicken Sie im Picker auf **FastComments**. Brightspace erstellt ein Thema im Modul und kehrt zur Content-Ansicht zurück.
5. Klicken Sie das neue Thema an. Benennen Sie es mit dem Inline-Titel-Editor in etwas Beschreibendes um, z. B. `FastComments Discussion`.

New Content Experience (Lessons):

1. Öffnen Sie den Kurs und klicken Sie auf **Content**.
2. Öffnen Sie die Unit und die Lesson, die die Diskussion enthalten soll.
3. Klicken Sie **Add** > **Existing Activity** und wählen Sie **FastComments** (älteres Brightspace: verschachtelt unter **External Learning Tools**).
4. Die Aktivität wird der Lesson hinzugefügt.
5. Klicken Sie den Aktivitätstitel an, um ihn umzubenennen.

Beim ersten Öffnen des Themas durch einen beliebigen Benutzer (Dozent oder Student) initialisiert FastComments den Thread für diesen Resource Link. Der Thread ist an die Resource Link ID gebunden, deshalb ändern Umbenennen oder Verschieben des Themas nicht, welcher Thread geladen wird.

#### FastComments inline in einem HTML-Thema einbetten

Verwenden Sie diesen Ablauf, wenn Kommentare unter einer Lektüre, einem Video oder anderem Inhalt auf derselben Themen-Seite und nicht als separates Thema erscheinen sollen.

1. Öffnen oder erstellen Sie ein HTML-Thema im Modul/der Lesson.
2. Klicken Sie **Edit HTML**, um den Brightspace-HTML-Editor zu öffnen.
3. Platzieren Sie den Cursor an der Stelle, an der der Kommentar-Thread erscheinen soll.
4. Klicken Sie die **Insert Stuff**-Schaltfläche (Puzzleteil-Symbol in der Editor-Symbolleiste).
5. Scrollen Sie im Insert-Stuff-Dialog zu **LTI Advantage** und klicken Sie **FastComments**.
6. FastComments öffnet einen Deep-Linking-Picker. Bestätigen Sie die Platzierung (die Standardoptionen funktionieren für Content-Diskussionen); klicken Sie **Insert** oder **Continue**.
7. Brightspace kehrt mit einem Platzhalterblock, der den LTI-Launch darstellt, in den HTML-Editor zurück. Klicken Sie **Save and Close** im Thema.

Beim Laden des Themas ersetzt Brightspace den Platzhalter durch ein iframe, das FastComments automatisch über LTI startet. Studierende sehen den Diskussions-Thread inline.

Ein einzelnes HTML-Thema kann mehrere deep-geteilte FastComments-Embed-Instanzen enthalten. Jede Einbettung erhält ihren eigenen Thread, weil jeder Deep Link eine eigene Resource Link ID erzeugt.

#### Modul-Thema vs. Inline-Quicklink

Wählen Sie den Ansatz "Modul-Thema", wenn:

- Die Diskussion die primäre Aktivität für diesen Schritt im Modul ist.
- Sie möchten, dass das Thema im Inhaltsverzeichnis von Brightspace, bei der Abschlussverfolgung und in Class Progress erscheint.

Wählen Sie den Ansatz "Inline-Embed", wenn:

- Kommentare unter anderem Inhalt auf derselben Seite stehen sollen.
- Sie keinen separaten, in der TOC abgehakten Abschlusspunkt wünschen.

#### Sichtbarkeit, Entwurf und Freigabebedingungen

Ein neues FastComments-Thema ist standardmäßig für Studierende sichtbar. Um es während der Einrichtung zu verbergen:

1. Klicken Sie im Content-Editor auf den Thema-Titel (Classic) oder das Drei-Punkte-Menü der Aktivität (New Content Experience).
2. Setzen Sie den Status auf **Draft** (Classic) oder schalten Sie die **Visibility** aus (New Content Experience).

Entwurfs-Themen sind für Studierende unsichtbar. Lehrende und TAs sehen sie weiterhin mit einem "Draft"-Badge.

Um das Thema auf eine bestimmte Gruppe oder Sektion zu beschränken:

1. Öffnen Sie das Thema.
2. Klicken Sie das Titel-Menü des Themas > **Edit Properties In-place** (Classic) oder **Edit** > **Restrictions** (New Content Experience).
3. Unter **Release Conditions** klicken Sie **Create**.
4. Wählen Sie **Group enrollment** oder **Section enrollment**, wählen Sie die Gruppe/Sektion aus und speichern Sie.

Freigabebedingungen stapeln sich mit den eigenen Rollenzuordnungen von FastComments. Studierende, die das Thema nicht sehen können, erhalten keinen LTI-Start.

#### Was Studierende beim ersten Start sehen

Wenn ein Studierender das Thema anklickt (oder ein HTML-Thema mit Embed lädt):

1. Führt Brightspace im Hintergrund den LTI 1.3-Launch durch.
2. Erhält FastComments den Namen, die E-Mail, die Avatar-URL und die LMS-Rolle des Studierenden und meldet ihn automatisch an. Es erscheint keine FastComments-Anmeldemaske.
3. Der Kommentar-Thread für diesen Resource Link wird innerhalb des Brightspace-iframes gerendert.

Rollenzuordnung beim Launch:

- Brightspace `Administrator` wird für den Thread in FastComments zu einem **admin** (volle Moderation, Löschen, Sperren und Konfigurationszugriff).
- Brightspace `Instructor` wird zu einem FastComments **moderator** (pin, hide, delete, ban).
- Alle anderen Rollen (`Learner`, `TeachingAssistant`, etc.) werden zu Standard-Kommentierenden.

Kommentare werden dem Brightspace-Konto des Studierenden zugeordnet. Wenn der Studierende seinen Namen oder Avatar in Brightspace ändert, synchronisiert der nächste LTI-Launch die Änderung.

#### Öffentlichen Zugriff sperren (empfohlen)

Standardmäßig sind FastComments-Kommentardaten öffentlich lesbar. Jede Person, die die URL oder ein API-Endpunkt eines Threads errät, kann dessen Kommentare sehen, auch außerhalb von Brightspace. Für Kursdiskussionen möchten Sie sehr wahrscheinlich die Ansicht auf eingeschriebene Lernende beschränken.

Öffnen Sie Ihre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget-Anpassungsseite</a> und erstellen Sie eine Regel mit aktivierter Option **Require SSO To View Comments**, und setzen Sie dann das Sicherheitsniveau auf **Secure SSO**, sodass Threads nur über den signierten LTI-Launch geladen werden können.

Siehe [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) für die vollständige Schritt-für-Schritt-Anleitung, einschließlich wie die Regel auf eine einzelne Domain oder Seite begrenzt werden kann.

#### Iframe-Höhe und Resize

FastComments sendet die `org.imsglobal.lti.frameResize`-postMessage bei jedem Thread-Render und bei Inhaltsänderungen (neuer Kommentar, Antworten erweitern). Brightspace hört auf diese Nachricht und passt die iframe-Höhe an, damit der Thread nicht abgeschnitten wird und kein innerer Scrollbalken erscheint.

Wenn das iframe auf einer festen niedrigen Höhe bleibt:

- Bestätigen Sie, dass der Kurs über HTTPS geladen wird. Brightspace’ postMessage-Listener lehnt Mixed-Content-Frames ab.
- Bestätigen Sie, dass keine Browser-Erweiterung den postMessage-Kanal blockiert.
- Bei Inline-Embeds in einem HTML-Thema darf das umgebende HTML das iframe nicht in einen Container mit fester Höhe einbetten. Entfernen Sie jegliches inline `style="height: ..."` vom Elternelement.

#### Brightspace-spezifische Fallstricke

**Tool wird im Add Existing-Picker nicht angezeigt.** Die Bereitstellung ist für die Organisations-Einheit dieses Kurses nicht aktiviert. Ein Administrator muss die Org-Einheit (oder eine übergeordnete Einheit) zur Liste der Org Units der Bereitstellung hinzufügen. Die Tool-Registrierung allein reicht nicht aus; die Bereitstellung legt fest, welche Kurse das Tool sehen.

**`deployment_id`-Mismatch beim Start.** FastComments pinnt initial die erste `deployment_id`, die es für eine Registrierung sieht. Wenn ein Administrator die ursprüngliche Bereitstellung löscht und eine neue erstellt, werden Starts von der neuen Bereitstellung mit einem Deployment-Mismatch-Fehler abgelehnt. Die Lösung ist, FastComments erneut zu registrieren (generieren Sie eine neue Registrierungs-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) und führen Sie die dynamische Registrierung erneut durch); der alte Konfigurationsdatensatz wird ersetzt.

**Tool startet, zeigt aber "Invalid LTI launch".** Der Kurs befindet sich in einer anderen Mandanten-/Organisationsstruktur als die, die die Bereitstellung abdeckt, oder die Bereitstellung wurde nach der Registrierung deaktiviert. Überprüfen Sie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-Schalter und die Org-Einheiten-Liste der Bereitstellung.

**Namen und Rollen fehlen in FastComments.** Brightspace sendet LTI-Starts mit Names and Role Provisioning Services (NRPS)-Claims. Wenn ein Kurs aus einem älteren LTI 1.1-Link migriert wurde, fehlen dem Start die `name`- und `email`-Claims. Fügen Sie das FastComments-Thema erneut über **Add Existing** hinzu (migrieren Sie nicht den alten Link), damit der Start LTI 1.3 verwendet.

**Embed zeigt einen Login-Bildschirm statt Auto-SSO.** Das HTML-Thema wurde als einfaches `<iframe>` eingefügt, das auf FastComments zeigt, anstatt über **Insert Stuff** > **LTI Advantage**. Einfache iframes überspringen den LTI-Launch und landen auf der öffentlich zugänglichen FastComments-Seite. Löschen Sie das iframe und fügen Sie es erneut über den Insert-Stuff-Flow ein.
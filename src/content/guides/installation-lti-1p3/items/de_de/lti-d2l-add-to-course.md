Diese Seite behandelt das Hinzufügen von FastComments zu einem Brightspace-Kurs, nachdem ein Administrator das Tool registriert und eine Bereitstellung erstellt hat. Wenn das Tool noch nicht registriert ist, lesen Sie zuerst die D2L-Registrierungsanleitung.

Brightspace liefert zwei Content-Authoring-Erfahrungen: **Classic Content** und die **New Content Experience** (auch **Lessons** genannt). Beide stellen FastComments zur Verfügung, jedoch unterscheiden sich die Menüpunkte. Jeder Abschnitt unten behandelt beide Varianten, wo sie auseinandergehen.

#### Locate the FastComments Tool

Das FastComments-Tool erscheint an zwei Stellen im Content-Editor eines Kurses:

1. Im Aktivitätsauswähler, erreichbar über die Schaltfläche **Add Existing** eines Moduls/Units (in älteren Brightspace-Versionen mit der Beschriftung **Add Existing Activities**). FastComments erscheint in aktuellen Brightspace-Versionen direkt im Picker; ältere Versionen listen es unter einem **External Learning Tools**-Untermenü. Beide Pfade fügen FastComments als eigenständiges Thema hinzu.
2. Im **Insert Stuff**-Dialog innerhalb des HTML-Editors, unter **LTI Advantage**. Dadurch wird FastComments mittels des LTI-Deep-Linking-Flows inline in ein HTML-Thema eingebettet.

Wenn FastComments in keinem der Picker erscheint, ist die Bereitstellung für die Organisationseinheit, die den Kurs enthält, nicht aktiviert. Bitten Sie Ihren Brightspace-Administrator, **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** zu öffnen, die Bereitstellung zu öffnen und die Organisationseinheit des Kurses (oder eine übergeordnete Organisationseinheit) unter **Org Units** hinzuzufügen.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Öffnen Sie den Kurs und klicken Sie in der Navigationsleiste auf **Content**.
2. Wählen Sie das Modul aus, das die Diskussion enthalten soll (oder erstellen Sie eines über **Add a module**).
3. Klicken Sie auf **Add Existing** (älteres Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klicken Sie im Picker auf **FastComments**. Brightspace erstellt ein Thema im Modul und bringt Sie zurück zur Inhaltsansicht.
5. Klicken Sie auf das neue Thema. Benennen Sie es mit dem Inline-Titel-Editor in etwas Beschreibendes wie `FastComments Discussion` um.

New Content Experience (Lessons):

1. Öffnen Sie den Kurs und klicken Sie auf **Content**.
2. Öffnen Sie die Unit und die Lesson, die die Diskussion enthalten sollen.
3. Klicken Sie auf **Add** > **Existing Activity** und wählen Sie **FastComments** (älteres Brightspace: verschachtelt unter **External Learning Tools**).
4. Die Aktivität wird der Lesson hinzugefügt.
5. Klicken Sie auf den Aktivitätstitel, um ihn umzubenennen.

Beim ersten Öffnen des Themas durch einen Benutzer (Dozent oder Student) initialisiert FastComments den Thread für diesen Resource Link. Der Thread ist an die Resource Link ID gebunden, sodass ein Umbenennen oder Verschieben des Themas nicht ändert, welcher Thread geladen wird.

#### Embed FastComments Inline in an HTML Topic

Verwenden Sie diesen Ablauf, wenn Kommentare unter einem Text, Video oder anderem Inhalt innerhalb derselben Themenseite statt als separates Thema erscheinen sollen.

1. Öffnen oder erstellen Sie ein HTML-Thema im Modul/Lesson.
2. Klicken Sie auf **Edit HTML**, um den Brightspace-HTML-Editor zu öffnen.
3. Platzieren Sie den Cursor an der Stelle, an der der Kommentar-Thread erscheinen soll.
4. Klicken Sie auf die Schaltfläche **Insert Stuff** (Puzzle-Symbol in der Editor-Symbolleiste).
5. Scrollen Sie im Insert Stuff-Dialog zu **LTI Advantage** und klicken Sie auf **FastComments**.
6. FastComments öffnet einen Deep-Linking-Picker. Bestätigen Sie die Platzierung (die Standardoptionen eignen sich für Inhaltsdiskussionen); klicken Sie auf **Insert** oder **Continue**.
7. Brightspace kehrt zum HTML-Editor mit einem Platzhalterblock zurück, der den LTI-Launch darstellt. Klicken Sie auf **Save and Close** im Thema.

Beim Laden des Themas ersetzt Brightspace den Platzhalter durch ein iframe, das FastComments automatisch über LTI startet. Studierende sehen den Diskussions-Thread inline.

Ein einzelnes HTML-Thema kann mehrere Deep-Linked FastComments-Embeds enthalten. Jedes Embed erhält seinen eigenen Thread, da jeder Deep Link eine eindeutige Resource Link ID erzeugt.

#### Module Topic vs Inline Quicklink

Wählen Sie den Ansatz "module topic", wenn:

- Die Diskussion die primäre Aktivität für diesen Schritt im Modul ist.
- Sie möchten, dass das Thema im Inhaltsverzeichnis von Brightspace, im Completion-Tracking und in Class Progress erscheint.

Wählen Sie den Ansatz "inline embed", wenn:

- Kommentare unter anderem Inhalt auf derselben Seite stehen sollen.
- Sie keinen separaten, nach Abschluss verfolgbaren Eintrag im Inhaltsverzeichnis wünschen.

#### Visibility, Draft, and Release Conditions

Ein neues FastComments-Thema ist standardmäßig für Studierende sichtbar. Um es während der Einrichtung zu verbergen:

1. Klicken Sie im Content-Editor auf den Titel des Themas (Classic) oder auf das Drei-Punkte-Menü der Aktivität (New Content Experience).
2. Setzen Sie den Status auf **Draft** (Classic) oder schalten Sie die **Visibility** aus (New Content Experience).

Draft-Themen sind für Studierende unsichtbar. Dozierende und TAs sehen sie weiterhin mit einem "Draft"-Badge.

Um das Thema auf eine bestimmte Gruppe oder Sektion zu beschränken:

1. Öffnen Sie das Thema.
2. Klicken Sie auf das Titemenü des Themas > **Edit Properties In-place** (Classic) oder **Edit** > **Restrictions** (New Content Experience).
3. Unter **Release Conditions** klicken Sie auf **Create**.
4. Wählen Sie **Group enrollment** oder **Section enrollment**, wählen Sie die Gruppe/Sektion aus und speichern Sie.

Freigabebedingungen werden mit FastComments eigener Rollenzuordnung kombiniert. Studierende, die das Thema nicht sehen können, erhalten keinen LTI-Launch.

#### What Students See on First Launch

Wenn ein Studierender das Thema anklickt (oder ein HTML-Thema mit einem Embed lädt):

1. Brightspace führt den LTI 1.3-Launch im Hintergrund durch.
2. FastComments empfängt den Namen, die E-Mail, die Avatar-URL und die LMS-Rolle des Studierenden und meldet ihn automatisch an. Es gibt keine FastComments-Anmeldeaufforderung.
3. Der Kommentar-Thread für diesen Resource Link rendert innerhalb des Brightspace-iframe.

Rollenzuordnung beim Launch:

- Brightspace `Administrator` wird für den Thread zu einem FastComments **admin** (vollständige Moderation, Löschen, Sperren und Konfigurationszugriff).
- Brightspace `Instructor` wird zu einem FastComments **moderator** (pin, hide, delete, ban).
- Alle anderen Rollen (`Learner`, `TeachingAssistant`, etc.) werden zu Standard-Kommentierenden.

Kommentare werden dem Brightspace-Konto des Studierenden zugeordnet. Wenn der Studierende seinen Namen oder Avatar in Brightspace ändert, synchronisiert der nächste LTI-Launch die Änderung.

#### Iframe Height and Resize

FastComments sendet die `org.imsglobal.lti.frameResize` postMessage bei jedem Thread-Rendering und bei Inhaltsänderungen (neuer Kommentar, aufgeklappte Antworten). Brightspace hört auf diese Nachricht und passt die iframe-Höhe an, sodass der Thread nicht abgeschnitten wird und kein innerer Scrollbalken angezeigt wird.

Wenn das iframe auf einer festen kurzen Höhe verbleibt:

- Bestätigen Sie, dass der Kurs über HTTPS geladen wird. Brightspace's postMessage-Listener lehnt Mixed-Content-Frames ab.
- Stellen Sie sicher, dass keine Browser-Erweiterung den postMessage-Kanal blockiert.
- Bei Inline-Embeds in einem HTML-Thema darf das umgebende HTML das iframe nicht in einen Container mit fester Höhe einschließen. Entfernen Sie jegliches inline `style="height: ..."` des Elternelements.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Die Bereitstellung ist für die Org Unit dieses Kurses nicht aktiviert. Ein Administrator muss die Org Unit (oder eine übergeordnete) zur Org Units-Liste der Bereitstellung hinzufügen. Die reine Tool-Registrierung reicht nicht aus; die Bereitstellung legt fest, welche Kurse das Tool sehen.

**`deployment_id` mismatch on launch.** FastComments bindet (TOFU) das erste `deployment_id`, das es für eine Registrierung sieht. Wenn ein Administrator die ursprüngliche Bereitstellung löscht und eine neue erstellt, werden Starts von der neuen Bereitstellung mit einem Deployment-Mismatch-Fehler abgelehnt. Die Lösung ist, FastComments neu zu registrieren (eine neue Registrierungs-URL zu erzeugen und die Dynamic Registration erneut auszuführen); der alte Konfigurationsdatensatz wird ersetzt.

**Tool launches but shows "Invalid LTI launch".** Der Kurs befindet sich in einer anderen Mandanten-/Organisationsstruktur als die Bereitstellung abdeckt, oder die Bereitstellung wurde nach der Registrierung deaktiviert. Überprüfen Sie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-Schalter und die Org Unit-Liste der Bereitstellung.

**Names and roles missing inside FastComments.** Brightspace liefert LTI-Starts mit Names and Role Provisioning Services (NRPS)-Claims. Wenn ein Kurs von einem älteren LTI 1.1-Link aktualisiert wurde, fehlen im Launch die `name`- und `email`-Claims. Fügen Sie das FastComments-Thema erneut über **Add Existing** hinzu (migrieren Sie den alten Link nicht), damit der Launch LTI 1.3 verwendet.

**Embed shows a login screen instead of auto-SSO.** Das HTML-Thema wurde als einfaches `<iframe>` eingefügt, das auf FastComments zeigt, statt über **Insert Stuff** > **LTI Advantage**. Einfache iframes überspringen den LTI-Launch und landen die Nutzer auf der öffentlich zugänglichen FastComments-Seite. Löschen Sie das iframe und fügen Sie es über den Insert Stuff-Flow erneut ein.
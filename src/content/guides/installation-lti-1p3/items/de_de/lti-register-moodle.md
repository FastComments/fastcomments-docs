**Verwenden Sie Moodle?** Wir veröffentlichen auch ein dediziertes Moodle-Plugin für FastComments mit einer engeren Integration als LTI 1.3 (Hooks zur Notensynchronisierung, umfassendere Aktivitätsberichte, native Moodle-Einstellungsoberfläche). Siehe das <a href="/guide-installation-moodle.html" target="_blank">Moodle-Plugin-Installationshandbuch</a>. Der untenstehende LTI 1.3-Flow ist die richtige Wahl, wenn Sie eine einzelne Registrierung wünschen, die auch andere LMS abdeckt, oder wenn Ihr Moodle-Administrator keine Drittanbieter-Plugins installieren möchte.

Moodle 4.0+ unterstützt die dynamische LTI 1.3-Registrierung über das External Tool-Plugin.

#### Öffnen Sie den Bildschirm zur Tool-Verwaltung

1. Melden Sie sich in Moodle als Seitenadministrator an.
2. Navigieren Sie zu **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Fügen Sie die URL ein

Sie sehen eine Karte mit der Bezeichnung **Tool URL**. Fügen Sie die FastComments-Registrierungs-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) in das Textfeld ein und klicken Sie auf **LTI Advantage hinzufügen**.

Moodle öffnet einen Registrierungsbildschirm, der die Identität des Tools und die angeforderten Berechtigungen anzeigt. Prüfen Sie diese und klicken Sie auf **Aktivieren** (oder **Registrieren**, je nach Moodle-Version).

Das Popup schließt sich, wenn die Registrierung abgeschlossen ist; das neue FastComments-Tool erscheint in der **Tools**-Liste mit dem Status **Active**.

#### Verfügbar machen

Standardmäßig fügt Moodle neue Tools der Liste "Course tools" hinzu, zeigt sie aber nicht im Aktivitätsauswahl-Dialog an. Um FastComments kursweit verfügbar zu machen:

1. Klicken Sie auf das Zahnrad-Symbol auf der FastComments-Kachel.
2. Wählen Sie unter **Tool configuration usage** die Option **Im Aktivitätsauswähler anzeigen und als vorkonfiguriertes Tool anzeigen**.
3. Speichern.

Dozierende können nun FastComments zu jedem Kurs hinzufügen über **Aktivität oder Material hinzufügen** > **FastComments**.
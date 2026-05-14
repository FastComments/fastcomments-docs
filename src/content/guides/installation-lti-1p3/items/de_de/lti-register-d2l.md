D2L Brightspace stellt die dynamische Registrierung über die LTI Advantage-Admin-Oberfläche bereit. Sie benötigen Administratorzugang.

#### Registrierungsbildschirm öffnen

1. Melden Sie sich bei Ihrer Brightspace-Instanz als Administrator an.
2. Navigieren Sie zu **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klicken Sie auf **Register Tool**. (Die direkte URL ist `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### URL einfügen

Sie sehen ein Registrierungsformular. Das Schlüsselfeld ist **Tool initiation registration endpoint** (einige Brightspace-Versionen bezeichnen es als „Tool Initiation Registration URL“).

Fügen Sie die FastComments-Registrierungs-URL in dieses Feld ein. Lassen Sie die anderen Felder leer – sie werden von FastComments während des Registrierungs-Handshakes automatisch ausgefüllt.

Klicken Sie auf **Register**.

#### Tool genehmigen

Brightspace öffnet ein Popup, das mit FastComments kommuniziert, Schlüssel austauscht und einen Bestätigungsbildschirm anzeigt. Das Popup schließt sich automatisch, wenn die Registrierung abgeschlossen ist.

Das neue Tool erscheint in Ihrer LTI Advantage-Toolliste. Standardmäßig markiert Brightspace neue Tools als **disabled** – schalten Sie den Schalter auf **enabled**, damit Ihre Kurse es verwenden können.

#### Eine Bereitstellung hinzufügen

In Brightspace benötigen LTI-Tools eine **deployment**, bevor sie in Kursen verwendet werden können:

1. Öffnen Sie das neu registrierte FastComments-Tool.
2. Klicken Sie auf **View Deployments** > **New Deployment**.
3. Geben Sie der Bereitstellung einen Namen (z. B. "FastComments - All Courses"), wählen Sie die Organisations-Einheiten, in denen sie verfügbar sein soll, und speichern Sie.

Nach dem ersten Start über diese Bereitstellung merkt sich FastComments die `deployment_id` in seinem Konfigurationsdatensatz – nachfolgende Starts aus einer anderen Bereitstellung desselben Clients werden abgewiesen, es sei denn, Sie registrieren erneut.
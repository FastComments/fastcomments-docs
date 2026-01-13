FastComments-Benutzerabzeichen werden von Administratoren mit der Berechtigung `Customize Data` konfiguriert.

Dies erfolgt über [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) in Ihrem Administrations-Dashboard.

Wenn einem Benutzer ein Abzeichen verliehen wird, wird es in seinem Profil und bei seinen Kommentaren angezeigt.

Beim Hinzufügen eines Abzeichens können wir ein `Display Label` festlegen, das der Name ist, den der Benutzer mit dem Abzeichen sieht. Zum Beispiel, wenn wir ein `Comment Count`-Abzeichen hinzufügen, möchten wir wahrscheinlich nicht diesen technischen Namen anzeigen, da er sehr langweilig ist. Wir könnten es `Super Member` oder ähnlich nennen. Abzeichen können sich auch stapeln und einander ersetzen, wie wir später in diesem Dokument behandeln werden.

Abzeichen haben außerdem konfigurierbare Schwellenwerte.

Abzeichen können erstellt und später durch Deaktivieren von `Enabled` deaktiviert werden. Das Deaktivieren eines Abzeichens bedeutet, dass es nicht mehr automatisch vergeben wird und nicht mehr im Menü Award Manual Badge angezeigt wird, aber Benutzer behalten das Abzeichen.

### Anzeigearten von Abzeichen

Abzeichen können Bild- oder Textabzeichen sein, die einige Basis-Styling-Optionen unterstützen (Textfarbe, Hintergrundfarbe und Rahmenfarbe). Sie können Abzeichen auch per CSS gestalten.

Bildabzeichen können GIF-Bilder sein, um animierte Abzeichen anzuzeigen.

### Tipp – Entfernen Sie Abzeichen nicht!

Benutzer lieben Abzeichen. Sie hängen oft sehr an ihnen, selbst wenn es ein Fehler war, den Sie versehentlich hinzugefügt haben, und Sie das Abzeichen-Icon ändern möchten.

Wenn wir etwas gelernt haben, dann dass es extrem schwierig ist, etwas von Benutzern wegzunehmen. Ein Abzeichen zu entfernen, nur weil es Ihnen als Site-Besitzer nicht mehr gefällt oder Sie Änderungen vornehmen möchten, kann eine sehr verärgerte Nutzergruppe zur Folge haben, die aus Frustration plötzlich Ihre Seite verlässt. Aus diesem Grund war `Delete` in den ersten Monaten nach der Veröffentlichung dieses Features nicht einmal als Option vorhanden – wir mussten es jedoch später hinzufügen. Bitte verwenden Sie Delete mit Vorsicht. Wir haben gesehen, dass viele langjährige, über Jahre aktive Nutzer sehr frustriert waren und ihre Communities verlassen haben, weil Administratoren beschlossen haben, ein Abzeichen zu löschen.

Wenn Sie ein Abzeichen nicht mehr verwenden müssen, empfehlen wir, es einfach zu deaktivieren, damit Benutzer ihr Abzeichen behalten.

### Abzeichen-Neuverarbeitung

Wenn ein Abzeichen hinzugefügt oder geändert wird, überprüft das System rückwirkend alle Personen, die mit Ihrer Seite interagiert haben, um festzustellen, ob sie das Abzeichen erhalten sollten. Dies wird auf der Badges-Seite im Admin-Dashboard sichtbar sein, da anstelle der Anzahl der Benutzer, die das Abzeichen haben, ein Spinner angezeigt wird. Das liegt daran, dass die Anzahl der Benutzer ermittelt wird.

### Anzeigen, wer ein Abzeichen hat

In der Abzeichenliste hat jeder Link die Option `View Users`, um die Liste der Benutzer anzuzeigen, die ein Abzeichen verdient haben oder dem es manuell verliehen wurde.
Wenn Sie Ihre Website von WordPress weg migrieren und FastComments auf der neuen Website verwenden möchten, benötigen Sie das WordPress-Plugin nicht. Exportieren Sie Ihre Kommentare aus WordPress und laden Sie die Datei dann auf der [Import-Seite](https://fastcomments.com/auth/my-account/manage-data/import) im FastComments-Dashboard hoch.

Wir unterstützen zwei WordPress-Exportformate.

### WordPress XML (Empfohlen)

Dies ist die Datei aus WordPresss integriertem Exportwerkzeug, daher wird kein zusätzliches Plugin benötigt.

1. Gehen Sie in Ihrer WordPress-Verwaltung zu `Tools -> Export`.
2. Wählen Sie `All content` aus und klicken Sie auf `Download Export File`.
3. Wählen Sie auf der FastComments-[Import-Seite](https://fastcomments.com/auth/my-account/manage-data/import) `WordPress (.xml)` aus und laden Sie die Datei hoch.

Jeder Kommentar ist mit der URL des Beitrags verknüpft, auf dem er hinterlassen wurde, die sich bereits in der Datei befindet.

Der Import behält den Autorennamen, die E-Mail und die Website, das Datum, den Inhalt, die Antwort-Verschachtelung und ob der Kommentar freigegeben wurde. Avatare der Kommentatoren werden von Gravatar übernommen. Stimmen sind nicht Teil dieses Formats.

### WordPress CSV

Dies ist die Datei aus dem [WebToffee WordPress Comments Import & Export Plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Installieren Sie das Plugin in Ihrer WordPress-Verwaltung und exportieren Sie Ihre Kommentare als CSV.
2. Ersetzen Sie jeden `comment_post_ID`-Wert durch die URL des Beitrags.
3. Wählen Sie auf der FastComments-[Import-Seite](https://fastcomments.com/auth/my-account/manage-data/import) `WordPress (.csv)` aus und laden Sie die Datei hoch.

Jeder Kommentar ist mit der Spalte `comment_post_ID` verknüpft. WordPress füllt diese Spalte mit der Beitrags-ID, und Ihre neue Website hat keine WordPress-Beitrags-IDs, daher ersetzt Schritt 2 sie durch die URL.

Der Import behält den Autorennamen, die E-Mail und die Website, das Datum, den Inhalt, die Antwort-Verschachtelung und ob der Kommentar freigegeben wurde. Avatare der Kommentatoren werden von Gravatar übernommen. Er behält auch WordPresss Spam-Markierung sowie wpDiscuz-Likes und -Dislikes, wenn die Datei diese enthält.

### Zuordnung von Kommentaren zu Ihren neuen Seiten

Wenn Ihre neue Website dieselben URLs wie Ihre WordPress-Website beibehält, erscheinen die Kommentare ohne zusätzliche Einrichtung auf den passenden Seiten.

Wenn sich die Domain ändert, führen Sie nach dem Import das [Domain-Migration-Werkzeug](/guide-migrations.html#migrating-domains) aus. Wenn sich einzelne Seiten-URLs ändern, können Sie [jede Seite migrieren](/guide-migrations.html#migrating-pages) – von der alten URL zur neuen.

Für Massenmigrationen von Seiten, z. B. das Entfernen der Domain aus dem Wert, den Sie an das Feld [urlId](/guide-customizations-and-configuration.html#url-id) des Kommentar-Widgets übergeben, [öffnen Sie ein Support-Ticket](https://fastcomments.com/auth/my-account/help) und wir kümmern uns für Sie darum.

### Vor dem Wechsel

Sie können den Import so oft ausführen, wie Sie möchten. Ein erneuter Import derselben Datei [erzeugt keine Duplikate](/guide-migrations.html#importing-data), sodass Sie einmal importieren können, um die neue Website zu testen, und dann kurz vor dem Wechsel erneut mit Ihren neuesten Kommentaren importieren.

Für Exportdateien größer als 1 GB, [wenden Sie sich an den Support](https://fastcomments.com/auth/my-account/help).

Um FastComments zu Ihrer neuen Website hinzuzufügen, siehe die [Installationsanleitung](/guide-installation.html).
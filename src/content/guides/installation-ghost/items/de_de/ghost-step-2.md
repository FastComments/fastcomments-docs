Nachdem wir die ZIP-Datei heruntergeladen haben, entpacken Sie sie in einen Ordner. Ich habe die Standarddatei `casper.zip` heruntergeladen und unter `Downloads\casper` unter Windows entpackt.

Als Nächstes sollten Sie sicherstellen, dass Sie die LTS- oder eine neuere Version von NodeJS installiert haben. Diese erhalten Sie hier: https://nodejs.org/en/download/

Sobald NodeJS installiert ist, sollten Sie einen Code-Editor installieren.

Wir empfehlen (und verwenden) Webstorm, das Sie hier mit einer 30-tägigen Testversion (keine Kreditkarte erforderlich) erhalten: https://www.jetbrains.com/webstorm/

Die nächstbeste kostenlose Alternative wäre wahrscheinlich Visual Studio Code: https://code.visualstudio.com/download

Sobald Ihr Editor eingerichtet ist und der Theme-Ordner im Editor geöffnet ist, öffnen Sie das Terminal in der IDE und führen Sie aus:

[inline-code-attrs-start title = 'Theme installieren'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Erfolgreiche Ausgabe sieht folgendermaßen aus (Warnungen können ignoriert werden):

<div class="screenshot white-bg">
    <div class="title">Erfolgreiche npm install-Ausgabe</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Erfolgreiche npm install-Ausgabe" />
</div>

Dies richtet die Abhängigkeiten des Themes für spätere Befehle ein, die wir ausführen werden. Außerdem hängt der Export davon ab, dass die Abhängigkeiten des Themes installiert sind; andernfalls funktioniert der Re-Import nicht korrekt.

---
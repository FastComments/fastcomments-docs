### Maven

Fügen Sie das Repsy-Repository zur POM Ihres Projekts hinzu:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Fügen Sie dann die benötigten Abhängigkeiten hinzu:

```xml
<dependencies>
    <!-- API-Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Kernbibliothek (enthält SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub-Bibliothek (für Live-Ereignisse) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Fügen Sie das Repsy-Repository zu Ihrer build.gradle-Datei hinzu:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API-Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Kernbibliothek (enthält SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub-Bibliothek (für Live-Ereignisse)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Bibliotheksinhalte

Diese Bibliothek enthält drei Module. Den generierten API-Client, die Core-Java-Bibliothek, die manuell geschriebene Hilfsfunktionen enthält, um die Arbeit mit der API zu erleichtern, und das `pubsub`-Modul, das eine Bibliothek zum Abonnieren von Änderungsfeeds ist.

- [API-Client-Bibliothek (Dokumentation)](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core-Bibliotheksdokumentation, einschließlich SSO-Beispiele](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Öffentliche vs. Gesicherte APIs

Für den API-Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und `PublicApi` enthält Methoden, die direkt aus einem Browser/Mobilgerät/etc. ohne Authentifizierung aufgerufen werden können.

Die `ModerationApi` treibt das Moderatoren-Dashboard an. Sie enthält Methoden zur Kommentar-Moderation (auflisten, zählen, suchen, Protokolle und Export), Moderationsaktionen (entfernen/wiederherstellen, markieren, Überprüfungs-/Spam-/Genehmigungsstatus setzen, Stimmen und Thread wieder öffnen/schließen), Sperren (vom Kommentieren ausschließen, Sperre rückgängig machen, Zusammenfassungen vor Sperren, Sperrstatus und -einstellungen sowie Anzahl gesperrter Benutzer) und Abzeichen & Vertrauen (Abzeichen vergeben/entfernen, manuelle Abzeichen, Vertrauensfaktor abrufen/setzen und internes Benutzerprofil). Jede `ModerationApi`-Methode akzeptiert einen `sso`-Parameter, sodass der Aufruf im Namen eines per SSO authentifizierten Moderators ausgeführt werden kann.
### Maven

Fügen Sie das Repsy-Repository der POM Ihres Projekts hinzu:

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
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core-Bibliothek (enthält SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub-Bibliothek (für Live-Ereignisse) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    implementation "com.fastcomments:client:1.3.2"
    
    // Core-Bibliothek (enthält SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub-Bibliothek (für Live-Ereignisse)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

Diese Bibliothek enthält drei Module. Den generierten API-Client, die Core-Java-Bibliothek, die handgeschriebene Hilfsfunktionen enthält, um die Arbeit mit der API zu erleichtern, und das `pubsub`-Modul, das eine Bibliothek zum Abonnieren von Änderungs-Feeds ist.

- [Dokumentation der API-Client-Bibliothek](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentation der Core-Bibliothek, einschließlich SSO-Beispielen](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentation der PubSub-Bibliothek](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Für den API-Client gibt es zwei Klassen, `DefaultApi` und `PublicApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel benötigen, und `PublicApi` enthält API-Aufrufe, die direkt von einem Browser/Mobilgerät/etc. ohne Authentifizierung durchgeführt werden können.
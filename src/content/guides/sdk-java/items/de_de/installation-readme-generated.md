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
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
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
    // API Client
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Inhalt der Bibliothek

Diese Bibliothek enthält drei Module. Den generierten API-Client, die Core-Java-Bibliothek, die manuell geschriebene Hilfsfunktionen enthält, um die Arbeit mit der API zu erleichtern, und das `pubsub`-Modul, welches eine Bibliothek zum Abonnieren von Änderungsfeeds ist.

- [API-Client-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentation der Core-Bibliothek, einschließlich SSO-Beispielen](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Öffentliche vs. geschützte APIs

Für den API-Client gibt es zwei Klassen, `DefaultApi` und `PublicApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel benötigen, und `PublicApi` enthält API-Aufrufe, die direkt aus einem Browser, einem Mobilgerät usw. ohne Authentifizierung ausgeführt werden können.
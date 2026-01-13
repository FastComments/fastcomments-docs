### Maven

Aggiungi il repository Repsy al POM del tuo progetto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Aggiungi quindi le dipendenze necessarie:

```xml
<dependencies>
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Libreria Core (include SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Libreria PubSub (per eventi in tempo reale) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Aggiungi il repository Repsy al file build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Client API
    implementation "com.fastcomments:client:0.0.2"
    
    // Libreria Core (include SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // Libreria PubSub (per eventi in tempo reale)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Contenuti della libreria

Questa libreria contiene tre moduli. Il client API generato, la libreria Java core che contiene utility scritte a mano per semplificare il lavoro con l'API, e il modulo `pubsub` che è una libreria per iscriversi ai feed delle modifiche.

- [Documentazione della libreria Client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentazione della libreria Core, inclusi esempi SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentazione della libreria PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API pubbliche vs protette

Per il client API, ci sono due classi, `DefaultApi` e `PublicApi`. `DefaultApi` contiene metodi che richiedono la tua chiave API, e `PublicApi` contiene chiamate API che possono essere effettuate direttamente da un browser/dispositivo mobile/etc. senza autenticazione.
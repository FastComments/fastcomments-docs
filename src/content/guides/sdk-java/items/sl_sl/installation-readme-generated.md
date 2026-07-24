### Maven

Dodajte Repsy repozitorij v POM vašega projekta:

```xml
<repositories>
    <repository>
  <id>repsy</id>
  <name>FastComments Maven Repository on Repsy</name>
  <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
 </repository>
</repositories>
```

Nato dodajte potrebne odvisnosti:

```xml
<dependencies>
 <!-- API Client -->
 <dependency>
   <groupId>com.fastcomments</groupId>
   <artifactId>client</artifactId>
   <version>3.0.1</version>
 </dependency>
 
 <!-- Core Library (includes SSO) -->
 <dependency>
   <groupId>com.fastcomments</groupId>
   <artifactId>core</artifactId>
   <version>3.0.1</version>
 </dependency>
 
 <!-- PubSub Library (for live events) -->
 <dependency>
   <groupId>com.fastcomments</groupId>
   <artifactId>pubsub</artifactId>
   <version>3.0.1</version>
 </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij v datoteko build.gradle:

```groovy
repositories {
 mavenCentral()
 maven {
   url "https://repo.repsy.io/mvn/winrid/fastcomments"
 }
}

dependencies {
 // API Client
 implementation "com.fastcomments:client:3.0.1"
 
 // Core Library (includes SSO)
 implementation "com.fastcomments:core:3.0.1"
 
 // PubSub Library (for live events)
 implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Vsebina knjižnice

Ta knjižnica vsebuje tri module. Generiran API odjemalec, jedrno Java knjižnico, ki vsebuje ročno napisane pripomočke za olajšanje dela z API-jem, in modul `pubsub`, ki je knjižnica za naročanje na spremembe virov.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija knjižnice PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs Zavarovani API-ji

Za API odjemalca so na voljo tri razrede, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` pa vsebuje metode, ki jih je mogoče klicati neposredno iz brskalnika/mobilne naprave/itd. brez avtentikacije.

`ModerationApi` ponuja obsežen nabor API-jev za živo in hitro moderiranje. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.
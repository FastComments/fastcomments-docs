### Maven

V POM datoteko vašega projekta dodajte Repsy repozitorij:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Nato dodajte odvisnosti, ki jih potrebujete:

```xml
<dependencies>
    <!-- API odjemalec -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Osnovna knjižnica (vključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub knjižnica (za dogodke v živo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

V datoteko build.gradle dodajte Repsy repozitorij:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API odjemalec
    implementation "com.fastcomments:client:3.1.2"
    
    // Osnovna knjižnica (vključuje SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub knjižnica (za dogodke v živo)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Vsebina knjižnice

Ta knjižnica vsebuje tri module. Generiranega odjemalca API-ja, osnovno Java knjižnico, ki vsebuje ročno napisane pripomočke
za lažje delo z API-jem, ter modul `pubsub`, ki je knjižnica za naročanje na vire sprememb.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija osnovne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija knjižnice PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni v primerjavi z zavarovanimi API-ji

Za odjemalca API-ja obstajajo trije razredi: `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` pa vsebuje metode,
ki jih je mogoče klicati neposredno iz brskalnika/mobilne naprave/ipd. brez overjanja.

`ModerationApi` ponuja obsežen nabor API-jev za moderiranje v živo in hitro moderiranje. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko overi prek SSO ali piškotka seje FastComments.com.
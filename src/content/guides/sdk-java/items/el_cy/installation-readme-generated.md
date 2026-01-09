### Maven

Προσθέστε το αποθετήριο Repsy στο POM του έργου σας:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Στη συνέχεια προσθέστε τις εξαρτήσεις που χρειάζεστε:

```xml
<dependencies>
    <!-- Πελάτης API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Βασική Βιβλιοθήκη (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για live γεγονότα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Προσθέστε το αποθετήριο Repsy στο αρχείο build.gradle σας:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Πελάτης API
    implementation "com.fastcomments:client:0.0.2"
    
    // Βασική Βιβλιοθήκη (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // Βιβλιοθήκη PubSub (για live γεγονότα)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τρεις μονάδες. Ο παραγόμενος πελάτης API, η βασική βιβλιοθήκη Java που περιέχει χειροποίητα βοηθητικά εργαλεία για να γίνει ευκολότερη η εργασία με το API, και το `pubsub` module που είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Δημόσια έναντι Ασφαλών API

Για τον πελάτη API, υπάρχουν δύο κλάσεις, `DefaultApi` και `PublicApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το API key σας, και η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.λπ. χωρίς αυθεντικοποίηση.
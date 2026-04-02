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
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // API Client
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τρεις μονάδες. Ο παραγόμενος πελάτης API, η βασική βιβλιοθήκη Java που περιλαμβάνει χειροποίητα βοηθητικά εργαλεία για να διευκολύνει την εργασία με το API, και η μονάδα `pubsub` η οποία είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Δημόσια vs Ασφαλή API

Για τον πελάτη API, υπάρχουν δύο κλάσεις, `DefaultApi` και `PublicApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.λπ. χωρίς αυθεντικοποίηση.
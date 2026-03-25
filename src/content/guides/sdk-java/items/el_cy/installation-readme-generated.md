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
        <version>1.3.1</version>
    </dependency>
    
    <!-- Βιβλιοθήκη Core (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για ζωντανά γεγονότα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    implementation "com.fastcomments:client:1.3.1"
    
    // Βιβλιοθήκη Core (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // Βιβλιοθήκη PubSub (για ζωντανά γεγονότα)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Περιεχόμενο Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τρία modules. Ο παραγόμενος πελάτης API, η βασική βιβλιοθήκη Java που περιέχει χειρόγραφα βοηθητικά εργαλεία για να γίνεται ευκολότερη η εργασία με το API, και το `pubsub` module το οποίο είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης API Client](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Core Library, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Δημόσια vs Προστατευμένα API

Για τον πελάτη API, υπάρχουν δύο κλάσεις, `DefaultApi` και `PublicApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.λπ. χωρίς αυθεντικοποίηση.
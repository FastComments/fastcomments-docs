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

Στη συνέχεια, προσθέστε τις εξαρτήσεις που χρειάζεστε:

```xml
<dependencies>
    <!-- Πελάτης API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Κύρια Βιβλιοθήκη (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για ζωντανά γεγονότα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Προσθέστε το αποθετήριο Repsy στο αρχείο **build.gradle** σας:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Πελάτης API
    implementation "com.fastcomments:client:3.0.1"
    
    // Κύρια Βιβλιοθήκη (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // Βιβλιοθήκη PubSub (για ζωντανά γεγονότα)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Library Contents

Αυτή η βιβλιοθήκη περιέχει τρία modules. Τον παραγόμενο πελάτη API, τη βασική βιβλιοθήκη Java που περιέχει χειρογράφους βοηθητικούς κώδικες για να διευκολύνει τη χρήση του API, και το module `pubsub` που είναι μια βιβλιοθήκη για την εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, ενώ η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από έναν φυλλομετρητή/συσκευή κινητής κίνησης κ.λπ. χωρίς έλεγχο ταυτότητας.

Η `ModerationApi` παρέχει ένα εκτενές σύνολο ζωντανών και γρήγορων API διαχείρισης. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή μέσω cookie συνεδρίας FastComments.com.
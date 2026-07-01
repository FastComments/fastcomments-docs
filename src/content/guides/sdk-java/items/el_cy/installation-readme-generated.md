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
        <version>2.0.0</version>
    </dependency>
    
    <!-- Βιβλιοθήκη Πυρήνα (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για ζωντανά γεγονότα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Προσθέστε το αποθετήριο Repsy στο αρχείο `build.gradle` σας:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Πελάτης API
    implementation "com.fastcomments:client:2.0.0"
    
    // Βιβλιοθήκη Πυρήνα (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // Βιβλιοθήκη PubSub (για ζωντανά γεγονότα)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Αυτή η βιβλιοθήκη περιέχει τρία modules. Το παραγόμενο API client, τη βασική Java βιβλιοθήκη που περιλαμβάνει χειρογράφως γραμμένα βοηθητικά εργαλεία για να κάνετε ευκολότερη τη χρήση του API, και το module `pubsub` που είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βιβλιοθήκης Πυρήνα, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Για τον API client, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, ενώ η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από πρόγραμμα περιήγησης/συσκευή κινητής κ.λπ. χωρίς έλεγχο ταυτότητας.

Η `ModerationApi` παρέχει μια εκτενή σειρά ζωντανών και γρήγορων API συντονισμού. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να αυθεντικοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.
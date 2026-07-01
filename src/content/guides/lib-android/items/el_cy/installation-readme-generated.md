Προσθέστε το FastComments SDK στο αρχείο `build.gradle.kts` της εφαρμογής σας:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Βεβαιωθείτε ότι έχετε το αποθετήριο Repsy στο `settings.gradle.kts` του έργου σας:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```
Dodajte FastComments SDK u `build.gradle.kts` fajl vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Uverite se da imate Repsy repozitorij u `settings.gradle.kts` vašeg projekta:

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
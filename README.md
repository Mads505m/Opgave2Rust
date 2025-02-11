# Opgave2Rust Lavet Af Peter, Mikkel, Magnus og Mads (5 mands gruppen)
 
Gør præcis om opgaven siger og forklar alt 
fektiv hukommelsesallokering og store datastrukturer

Du arbejder som udvikler for en virksomhed, der analyserer store mængder af sensor-data fra et netværk af IoT-enheder. Opgaven er at udvikle et program, der effektivt allokerer og håndterer data fra disse sensorer uden at overbelaste hukommelsen, og samtidig optimere ydeevnen.

Denne opgave skal illustrere forskellen i hukommelsesstyring mellem Rust og Java eller C#, hvor du opretter og administrerer store datastrukturer, som f.eks. vektorer, arrays og hashmaps.

Du skal udvikle et program i Rust, der indlæser og analyserer store datasæt effektivt ved hjælp af datastrukturer som Vec, HashMap og Array. Derefter sammenligner du ydeevnen med et tilsvarende program i Java eller C#, der bruger lignende datastrukturer som ArrayList og HashMap.

Del 1: Indlæsning af sensor-data i en vektor

1. Opret en vektor i Rust, der simulerer data fra 1 million sensorer. Hvert element i vektoren indeholder et sensor-ID og en værdi.

2. Brug vektoren til at gemme og analysere data.

3. Sammenlign hukommelsesforbrug og ydeevne med C# eller Java's ArrayList.


Del 2: Brug af HashMap til søgning af sensor-data

1. Opret en HashMap i Rust, der indeholder sensor-ID'er som nøgler og sensorværdier som værdier.

2. Implementer søgning efter et specifikt sensor-ID.

3. Sammenlign søge- og indsætningstider med med C# dictionary eller Java's HashMap.


Del 3: Hukommelsesoptimering med Box i Rust

1. Optimer hukommelsen i Rust ved hjælp af Box, som bruges til heap-allokering af store datastrukturer.

2. Forklar, hvordan Box hjælper med at minimere hukommelsesforbruget, og sammenlign med Java's håndtering af objekter på heapen.


Diskussionsspørgsmål:

Hvordan adskiller Rust's memory management sig fra Java’s garbage collection i forhold til allokering af store datastrukturer?

Hvad er fordelene ved at bruge Box i Rust til store datastrukturer sammenlignet med Java's heap-allokering af objekter?

Hvordan påvirker hukommelsesforbrug og ydeevne, når vi arbejder med store mængder data i Rust sammenlignet med Java

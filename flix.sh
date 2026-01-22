#!/usr/bin/env bash

cd "$(dirname "$0")"

# Init database
rm flix.db
cargo run -- init


###############
# Collections #
###############

# DC
cargo run -- add flix collection "DC Collection" ""
cargo run -- add tmdb show 60708          # Gotham (2014)

# DCEU
cargo run -- add --sort-title "dceu" flix collection "Worlds of DC" "American media franchise and shared universe that is centered on a series of superhero films, distributed by Warner Bros. Pictures and based on characters that appear in American comic books by DC Comics."
cargo run -- add tmdb collection 573693   # Aquaman Collection
cargo run -- add tmdb movie 297802        # Aquaman (2018)
cargo run -- add tmdb movie 572802        # Aquaman and the Lost Kingdom (2023)

# Marvel
cargo run -- add --sort-title "marvel" flix collection "In Association With Marvel" "Movies based on Marvel Comics properties not produced by Marvel Studios"
cargo run -- add --fs-slug "cloak and dagger (2018)" --web-slug "cloak-and-dagger-2018" tmdb show 66190            # Marvel's Cloak & Dagger (2018)
cargo run -- add --fs-slug "daredevil (2015)" --web-slug "daredevil-2015" tmdb show 61889                          # Marvel's Daredevil (2015)
cargo run -- add --fs-slug "inhumans (2017)" --web-slug "inhumans-2017" tmdb show 68716                            # Marvel's Inhumans (2017)
cargo run -- add --fs-slug "iron fist (2017)" --web-slug "iron-fist-2017" tmdb show 62127                          # Marvel's Iron Fist (2017)
cargo run -- add --fs-slug "jessica jones (2015)" --web-slug "jessica-jones-2015" tmdb show 38472                  # Marvel's Jessica Jones (2015)
cargo run -- add --fs-slug "luke cage (2016)" --web-slug "luke-cage-2016" tmdb show 62126                          # Marvel's Luke Cage (2016)
cargo run -- add --fs-slug "runaways (2017)" --web-slug "runaways-2017" tmdb show 67466                            # Marvel's Runaways (2017)
cargo run -- add --fs-slug "defenders (2017)" --web-slug "defenders-2017" tmdb show 62285                          # Marvel's The Defenders (2017)
cargo run -- add --fs-slug "punisher (2017)" --web-slug "punisher-2017" tmdb show 67178                            # Marvel's The Punisher (2017)

# Marvel Cinematic Universe
cargo run -- add flix collection "Marvel Cinematic Universe" ""
cargo run -- add tmdb show 84958          # Loki (2021)
cargo run -- add --fs-slug "agent carter (2015)" --web-slug "agent-carter-2015" tmdb show 61550                  # Marvel's Agent Carter (2015)
cargo run -- add --fs-slug "agents of shield (2013)" --web-slug "agents-of-shield-2013" tmdb show 1403           # Marvel's Agents of S.H.I.E.L.D. (2013)

# Star Wars
cargo run -- add tmdb collection 10       # Star Wars Collection
cargo run -- add  --title "Star Wars: Episode I - The Phantom Menace" --sort-title "star wars 1 - the phantom menace" --fs-slug "the phantom menace (1999)" --web-slug "star-wars-the-phantom-menace-1999" tmdb movie 1893                               # Star Wars: Episode I - The Phantom Menace (1999)
cargo run -- add  --title "Star Wars: Episode II - Attack of the Clones" --sort-title "star wars 2 - attack of the clones" --fs-slug "attack of the clones (2002)" --web-slug "star-wars-attack-of-the-clones-2002" tmdb movie 1894                      # Star Wars: Episode II - Attack of the Clones (2002)
cargo run -- add  --title "Star Wars: Episode III - Revenge of the Sith" --sort-title "star wars 3 - revenge of the sith" --fs-slug "revenge of the sith (2005)" --web-slug "star-wars-revenge-of-the-sith-2005" tmdb movie 1895                         # Star Wars: Episode III - Revenge of the Sith (2005)
cargo run -- add  --title "Star Wars: Episode IV - A New Hope" --sort-title "star wars 4 - a new hope" --fs-slug "a new hope (1977)" --web-slug "star-wars-a-new-hope-1977" tmdb movie 11                                                                # Star Wars (1977)
cargo run -- add  --title "Star Wars: Episode V - The Empire Strikes Back" --sort-title "star wars 5 - the empire strikes back" --fs-slug "the empire strikes back (1980)" --web-slug "star-wars-the-empire-strikes-back-1980" tmdb movie 1891           # The Empire Strikes Back (1980)
cargo run -- add  --title "Star Wars: Episode VI - Return of the Jedi" --sort-title "star wars 6 - return of the jedi" --fs-slug "return of the jedi (1983)" --web-slug "star-wars-return-of-the-jedi-1983" tmdb movie 1892                              # Return of the Jedi (1983)
cargo run -- add  --title "Star Wars: Episode VII - The Force Awakens" --sort-title "star wars 7 - the force awakens" --fs-slug "the force awakens (2015)" --web-slug "star-wars-the-force-awakens-2015" tmdb movie 140607                               # Star Wars: The Force Awakens (2015)
cargo run -- add  --title "Star Wars: Episode VIII - The Last Jedi" --sort-title "star wars 8 - the last jedi" --fs-slug "the last jedi (2017)" --web-slug "star-wars-the-last-jedi-2017" tmdb movie 181808                                              # Star Wars: The Last Jedi (2017)
cargo run -- add  --title "Star Wars: Episode IX - The Rise of Skywalker" --sort-title "star wars 9 - the rise of skywalker" --fs-slug "the rise of skywalker (2019)" --web-slug "star-wars-the-rise-of-skywalker-2019" tmdb movie 181812                # Star Wars: The Rise of Skywalker (2019)
cargo run -- add tmdb show 82856          # The Mandalorian (2019)

# Twin Peaks
cargo run -- add flix collection "Twin Peaks Collection" ""
cargo run -- add tmdb show 1920           # Twin Peaks (1990)


#####################
# Movie Collections #
#####################

# Disney Live-Action Remakes
cargo run -- add flix collection "Disney Live-Action Remakes" "Live-action or photorealistic remakes produced by Walt Disney Pictures of its animated films."
cargo run -- add tmdb movie 447273        # Snow White (2025)

# Happy Gilmore
cargo run -- add tmdb collection 1263259  # Happy Gilmore Collection
cargo run -- add tmdb movie 9614          # Happy Gilmore (1996)
cargo run -- add tmdb movie 1263256       # Happy Gilmore 2 (2025)

# Minecraft
cargo run -- add tmdb collection 1461530  # The Minecraft Movie Collection
cargo run -- add tmdb movie 950387        # A Minecraft Movie (2025)


####################
# Show Collections #
####################

# Arrowverse
cargo run -- add flix collection "Arrowverse" "A television franchise that is based on characters that appear in publications by DC Comics."
cargo run -- add tmdb show 1412           # Arrow (2012)
cargo run -- add tmdb show 89247          # Batwoman (2019)
cargo run -- add tmdb show 71663          # Black Lightning (2018)
cargo run -- add tmdb show 60735          # The Flash (2014)
cargo run -- add --fs-slug "legends of tomorrow (2016)" --web-slug "legends-of-tomorrow-2016" tmdb show 62643
cargo run -- add tmdb show 62688          # Supergirl (2015)

# Avatar Universe
cargo run -- add flix collection "Avatar Universe" "Avatar: The Last Airbender is set in an Asiatic-like world in which some people can manipulate the classical elements with psychokinetic variants of the Chinese martial arts known as 'bending'."
cargo run -- add tmdb show 246            # Avatar: The Last Airbender (2005)
cargo run -- add tmdb show 33880          # The Legend of Korra (2012)

# Breaking Bad
cargo run -- add flix collection "Breaking Bad Collection" "Collection containing the original Breaking Bad show along with a documentary chronicling the process of making the final season, a sequel movie and prequel show."
cargo run -- add tmdb show 1396           # Breaking Bad (2008)

# Buffyverse
cargo run -- add flix collection "Buffyverse" "The Buffyverse is a setting in which supernatural phenomena exist, and supernatural evil can be challenged by people willing to fight against such forces."
cargo run -- add tmdb show 2426           # Angel (1999)
cargo run -- add tmdb show 95             # Buffy the Vampire Slayer (1997)


##########
# Movies #
##########
cargo run -- add tmdb movie 940551        # Migration (2023)


#########
# Shows #
#########
cargo run -- add tmdb show 62110          # Assassination Classroom (2015)
cargo run -- add tmdb show 1429           # Attack on Titan (2013)
cargo run -- add tmdb show 42009          # Black Mirror (2011)
cargo run -- add tmdb show 1911           # Bones (2005)
cargo run -- add tmdb show 48891          # Brooklyn Nine-Nine (2013)
cargo run -- add flix episode "brooklyn-nine-nine-2013" 8 10 "The Last Day (Part 2)" "The squad takes stock of its eight years together and looks toward the future." "2021-09-16"
cargo run -- add tmdb show 3787           # Chaotic (2006)
cargo run -- add tmdb show 2557           # Class of the Titans (2006)
cargo run -- add tmdb show 13916          # Death Note (2006)
cargo run -- add tmdb show 1405           # Dexter (2006)
cargo run -- add tmdb show 1399           # Game of Thrones (2011)
cargo run -- add tmdb show 40075          # Gravity Falls (2012)
cargo run -- add tmdb show 1639           # Heroes (2006)
cargo run -- add tmdb show 60858          # Heroes Reborn (2015)
cargo run -- add tmdb show 71340          # Krypton (2018)
cargo run -- add tmdb show 62687          # Limitless (2015)
cargo run -- add tmdb show 60846          # Log Horizon (2013)
cargo run -- add tmdb show 64432          # The Magicians (2015)
cargo run -- add tmdb show 5920           # The Mentalist (2008)
cargo run -- add tmdb show 12786          # Murdoch Mysteries (2008)
cargo run -- add tmdb show 65930          # My Hero Academia (2016)
cargo run -- add tmdb show 2288           # Prison Break (2005)
cargo run -- add tmdb show 95396          # Severance (2022)
cargo run -- add tmdb show 60573          # Silicon Valley (2014)
cargo run -- add tmdb show 37680          # Suits (2011)
cargo run -- add tmdb show 45782          # Sword Art Online (2012)
cargo run -- add tmdb show 48860          # The Tomorrow People (2013)
cargo run -- add tmdb show 46331          # Under the Dome (2013)
cargo run -- add tmdb show 1432           # Veronica Mars (2004)
cargo run -- add tmdb show 186            # Weeds (2005)
cargo run -- add tmdb show 63247          # Westworld (2016)
cargo run -- add tmdb show 71912          # The Witcher (2019)

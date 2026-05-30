#!/usr/bin/env bash

cd "$(dirname "$0")" || exit 1

# Init database
rm flix.db
flix init


###############
# Collections #
###############

# DC
flix add flix collection "DC Collection" ""
flix add tmdb collection 1025281                 # Constantine Collection
flix add tmdb movie 561                          # Constantine (2005)
flix add tmdb collection 263                     # The Dark Knight Collection
flix add tmdb movie 272                          # Batman Begins (2005)
flix add tmdb movie 155                          # The Dark Knight (2008)
flix add tmdb movie 49026                        # The Dark Knight Rises (2012)
flix add tmdb movie 44912                        # Green Lantern (2011)
flix add tmdb show 60708                         # Gotham (2014)
flix add tmdb show 71340                         # Krypton (2018)
flix add tmdb movie 13455                        # Push (2009)

# DCEU
flix add --sort-title "dceu" flix collection "Worlds of DC" "American media franchise and shared universe that is centered on a series of superhero films, distributed by Warner Bros. Pictures and based on characters that appear in American comic books by DC Comics."
flix add tmdb collection 573693                  # Aquaman Collection
flix add tmdb movie 297802                       # Aquaman (2018)
flix add tmdb movie 572802                       # Aquaman and the Lost Kingdom (2023)
flix add tmdb movie 495764                       # Birds of Prey (and the Fantabulous Emancipation of One Harley Quinn) (2020)
flix add tmdb collection 531242                  # Suicide Squad Collection
flix add tmdb movie 297761                       # Suicide Squad (2016)
flix add tmdb movie 436969                       # The Suicide Squad (2021)
flix add tmdb collection 468552                  # Wonder Woman Collection
flix add tmdb movie 297762                       # Wonder Woman (2017)
flix add tmdb movie 464052                       # Wonder Woman 1984 (2020)

# Marvel
flix add --sort-title "marvel" flix collection "In Association With Marvel" "Movies based on Marvel Comics properties not produced by Marvel Studios"
flix add --fs-slug "cloak and dagger (2018)" --web-slug "cloak-and-dagger-2018" tmdb show 66190                         # Marvel's Cloak & Dagger (2018)
flix add --fs-slug "daredevil (2015)" --web-slug "daredevil-2015" tmdb show 61889                                       # Marvel's Daredevil (2015)
flix add --fs-slug "inhumans (2017)" --web-slug "inhumans-2017" tmdb show 68716                                         # Marvel's Inhumans (2017)
flix add --fs-slug "iron fist (2017)" --web-slug "iron-fist-2017" tmdb show 62127                                       # Marvel's Iron Fist (2017)
flix add --fs-slug "jessica jones (2015)" --web-slug "jessica-jones-2015" tmdb show 38472                               # Marvel's Jessica Jones (2015)
flix add --fs-slug "luke cage (2016)" --web-slug "luke-cage-2016" tmdb show 62126                                       # Marvel's Luke Cage (2016)
flix add --fs-slug "runaways (2017)" --web-slug "runaways-2017" tmdb show 67466                                         # Marvel's Runaways (2017)
flix add --fs-slug "defenders (2017)" --web-slug "defenders-2017" tmdb show 62285                                       # Marvel's The Defenders (2017)
flix add --fs-slug "punisher (2017)" --web-slug "punisher-2017" tmdb show 67178                                         # Marvel's The Punisher (2017)
flix add tmdb collection 125574                  # The Amazing Spider-Man Collection
flix add tmdb movie 1930                         # The Amazing Spider-Man (2012)
flix add tmdb movie 102382                       # The Amazing Spider-Man 2 (2014)
flix add tmdb collection 735                     # Blade Collection
flix add tmdb movie 36647                        # Blade (1998)
flix add tmdb movie 36586                        # Blade II (2002)
flix add tmdb movie 36648                        # Blade: Trinity (2004)
flix add tmdb collection 448150                  # Deadpool Collection
flix add tmdb movie 293660                       # Deadpool (2016)
flix add tmdb movie 383498                       # Deadpool 2 (2018)
flix add --fs-slug "deadpool and wolverine (2024)" --web-slug "deadpool-and-wolverine-2024" tmdb movie 533535                                                                   # Deadpool & Wolverine (2024)
flix add tmdb collection 9744                    # Fantastic Four Collection
flix add tmdb movie 9738                         # Fantastic Four (2005)
flix add tmdb movie 1979                         # Fantastic Four: Rise of the Silver Surfer (2007)
flix add tmdb movie 166424                       # Fantastic Four (2015)
flix add tmdb movie 1927                         # Hulk (2003)
flix add tmdb collection 179892                  # Kick-Ass Collection
flix add tmdb movie 23483                        # Kick-Ass (2010)
flix add tmdb movie 59859                        # Kick-Ass 2 (2013)
flix add tmdb movie 634492                       # Madame Web (2024)
flix add tmdb collection 556                     # Spider-Man Collection
flix add tmdb movie 557                          # Spider-Man (2002)
flix add tmdb movie 558                          # Spider-Man 2 (2004)
flix add tmdb movie 559                          # Spider-Man 3 (2007)
flix add tmdb collection 453993                  # The Wolverine Collection
flix add tmdb movie 2080                         # X-Men Origins: Wolverine (2009)
flix add tmdb movie 76170                        # The Wolverine (2013)
flix add tmdb movie 263115                       # Logan (2017)
flix add tmdb collection 748                     # X-Men Collection
flix add tmdb movie 36657                        # X-Men (2000)
flix add tmdb movie 36658                        # X2 (2003)
flix add tmdb movie 36668                        # X-Men: The Last Stand (2006)
flix add tmdb movie 49538                        # X-Men: First Class (2011)
flix add tmdb movie 127585                       # X-Men: Days of Future Past (2014)
flix add tmdb movie 246655                       # X-Men: Apocalypse (2016)
flix add tmdb movie 320288                       # Dark Phoenix (2019)
flix add tmdb collection 1645434                 # X-Anthology
flix add tmdb movie 26623                        # Generation X (1996)
flix add tmdb movie 340102                       # The New Mutants (2020)
flix add tmdb collection 558216                  # Venom Collection
flix add tmdb movie 335983                       # Venom (2018)
flix add tmdb movie 580489                       # Venom: Let There Be Carnage (2021)
flix add tmdb movie 912649                       # Venom: The Last Dance (2024)

# Marvel Cinematic Universe
flix add flix collection "Marvel Cinematic Universe" ""
flix add tmdb show 84958                         # Loki (2021)
flix add --fs-slug "agent carter (2015)" --web-slug "agent-carter-2015" tmdb show 61550                                 # Marvel's Agent Carter (2015)
flix add --fs-slug "agents of shield (2013)" --web-slug "agents-of-shield-2013" tmdb show 1403                          # Marvel's Agents of S.H.I.E.L.D. (2013)
flix add tmdb collection 422834                  # Ant-Man Collection
flix add tmdb movie 102899                       # Ant-Man (2015)
flix add tmdb movie 363088                       # Ant-Man and the Wasp (2018)
flix add tmdb movie 640146                       # Ant-Man and the Wasp: Quantumania (2023)
flix add tmdb collection 86311                   # The Avengers Collection
flix add tmdb movie 24428                        # The Avengers (2012)
flix add tmdb movie 99861                        # Avengers: Age of Ultron (2015)
flix add tmdb movie 299536                       # Avengers: Infinity War (2018)
flix add tmdb movie 299534                       # Avengers: Endgame (2019)
# flix add tmdb movie 1003596 # upcoming
# flix add tmdb movie 1003598 # upcoming
flix add tmdb collection 529892                  # Black Panther Collection
flix add tmdb movie 284054                       # Black Panther (2018)
flix add tmdb movie 505642                       # Black Panther: Wakanda Forever (2022)
flix add tmdb movie 497698                       # Black Widow (2021)
flix add tmdb collection 131295                  # Captain America Collection
flix add tmdb movie 1771                         # Captain America: The First Avenger (2011)
flix add tmdb movie 100402                       # Captain America: The Winter Soldier (2014)
flix add tmdb movie 271110                       # Captain America: Civil War (2016)
flix add tmdb movie 822119                       # Captain America: Brave New World (2025)
flix add tmdb collection 623911                  # Captain Marvel Collection
flix add tmdb movie 299537                       # Captain Marvel (2019)
flix add tmdb movie 609681                       # The Marvels (2023)
flix add tmdb collection 618529                  # Doctor Strange Collection
flix add tmdb movie 284052                       # Doctor Strange (2016)
flix add tmdb movie 453395                       # Doctor Strange in the Multiverse of Madness (2022)
flix add tmdb movie 524434                       # Eternals (2021)
flix add tmdb collection 284433                  # Guardians of the Galaxy Collection
flix add tmdb movie 118340                       # Guardians of the Galaxy (2014)
flix add tmdb movie 283995                       # Guardians of the Galaxy Vol. 2 (2017)
flix add tmdb movie 447365                       # Guardians of the Galaxy Vol. 3 (2023)
flix add tmdb collection 131292                  # Iron Man Collection
flix add tmdb movie 1726                         # Iron Man (2008)
flix add tmdb movie 10138                        # Iron Man 2 (2010)
flix add tmdb movie 68721                        # Iron Man 3 (2013)
flix add tmdb movie 566525                       # Shang-Chi and the Legend of the Ten Rings (2021)
flix add tmdb collection 531241                  # Spider-Man (MCU) Collection
flix add tmdb movie 315635                       # Spider-Man: Homecoming (2017)
flix add tmdb movie 429617                       # Spider-Man: Far From Home (2019)
flix add tmdb movie 634649                       # Spider-Man: No Way Home (2021)
# flix add tmdb movie 969681 # upcoming
flix add tmdb movie 617126                       # The Fantastic 4: First Steps (2025)
flix add tmdb movie 1724                         # The Incredible Hulk (2008)
flix add tmdb collection 131296                  # Thor Collection
flix add tmdb movie 10195                        # Thor (2011)
flix add tmdb movie 76338                        # Thor: The Dark World (2013)
flix add tmdb movie 284053                       # Thor: Ragnarok (2017)
flix add tmdb movie 616037                       # Thor: Love and Thunder (2022)
flix add tmdb movie 986056                       # Thunderbolts* (2025)

# South Park
flix add flix collection "South Park Collection" "Follow the misadventures of four irreverent grade-schoolers in the quiet, dysfunctional town of South Park, Colorado."
flix add tmdb show 2190                          # South Park (1997)
flix add --fs-slug "south park bigger longer and uncut (1999)" --web-slug "south-park-bigger-longer-and-uncut-1999" tmdb movie 9473                         # South Park: Bigger, Longer & Uncut (1999)
flix add tmdb collection 913392                  # South Park: Post COVID Collection
flix add tmdb movie 874299                       # South Park: Post COVID (2021)
flix add tmdb movie 874300                       # South Park: Post COVID: The Return of COVID (2021)
flix add tmdb collection 993732                  # South Park the Streaming Wars Collection
flix add tmdb movie 974691                       # South Park the Streaming Wars (2022)
flix add tmdb movie 993729                       # South Park the Streaming Wars Part 2 (2022)
flix add tmdb movie 1190012                      # South Park: Joining the Panderverse (2023)
flix add tmdb movie 1219926                      # South Park (Not Suitable for Children) (2023)
flix add tmdb movie 1290938                      # South Park: The End of Obesity (2024)

# Star Wars
flix add tmdb collection 10                      # Star Wars Collection
flix add --title "Star Wars: Episode I - The Phantom Menace" --sort-title "star wars 1 - the phantom menace" --fs-slug "the phantom menace (1999)" --web-slug "star-wars-the-phantom-menace-1999" tmdb movie 1893                                             # Star Wars: Episode I - The Phantom Menace (1999)
flix add --title "Star Wars: Episode II - Attack of the Clones" --sort-title "star wars 2 - attack of the clones" --fs-slug "attack of the clones (2002)" --web-slug "star-wars-attack-of-the-clones-2002" tmdb movie 1894                                    # Star Wars: Episode II - Attack of the Clones (2002)
flix add --title "Star Wars: Episode III - Revenge of the Sith" --sort-title "star wars 3 - revenge of the sith" --fs-slug "revenge of the sith (2005)" --web-slug "star-wars-revenge-of-the-sith-2005" tmdb movie 1895                                       # Star Wars: Episode III - Revenge of the Sith (2005)
flix add --title "Star Wars: Episode IV - A New Hope" --sort-title "star wars 4 - a new hope" --fs-slug "a new hope (1977)" --web-slug "star-wars-a-new-hope-1977" tmdb movie 11                                                                              # Star Wars (1977)
flix add --title "Star Wars: Episode V - The Empire Strikes Back" --sort-title "star wars 5 - the empire strikes back" --fs-slug "the empire strikes back (1980)" --web-slug "star-wars-the-empire-strikes-back-1980" tmdb movie 1891                         # The Empire Strikes Back (1980)
flix add --title "Star Wars: Episode VI - Return of the Jedi" --sort-title "star wars 6 - return of the jedi" --fs-slug "return of the jedi (1983)" --web-slug "star-wars-return-of-the-jedi-1983" tmdb movie 1892                                            # Return of the Jedi (1983)
flix add --title "Star Wars: Episode VII - The Force Awakens" --sort-title "star wars 7 - the force awakens" --fs-slug "the force awakens (2015)" --web-slug "star-wars-the-force-awakens-2015" tmdb movie 140607                                             # Star Wars: The Force Awakens (2015)
flix add --title "Star Wars: Episode VIII - The Last Jedi" --sort-title "star wars 8 - the last jedi" --fs-slug "the last jedi (2017)" --web-slug "star-wars-the-last-jedi-2017" tmdb movie 181808                                                            # Star Wars: The Last Jedi (2017)
flix add --title "Star Wars: Episode IX - The Rise of Skywalker" --sort-title "star wars 9 - the rise of skywalker" --fs-slug "the rise of skywalker (2019)" --web-slug "star-wars-the-rise-of-skywalker-2019" tmdb movie 181812                              # Star Wars: The Rise of Skywalker (2019)
flix add tmdb show 82856                         # The Mandalorian (2019)
flix add tmdb movie 330459                       # Rogue One: A Star Wars Story (2016)
flix add tmdb movie 348350                       # Solo: A Star Wars Story (2018)

# Twin Peaks
flix add flix collection "Twin Peaks Collection" ""
flix add tmdb show 1920                          # Twin Peaks (1990)


#####################
# Movie Collections #
#####################

# Airplane
flix add tmdb collection 91663                   # Airplane Collection
flix add tmdb movie 813                          # Airplane! (1980)
flix add tmdb movie 2665                         # Airplane II: The Sequel (1982)

# Alice in Wonderland
flix add tmdb collection 261307                  # Alice in Wonderland Collection
flix add tmdb movie 12155                        # Alice in Wonderland (2010)
flix add tmdb movie 241259                       # Alice Through the Looking Glass (2016)

# Anastasia
flix add tmdb collection 920650                  # Anastasia Collection
flix add tmdb movie 9444                         # Anastasia (1997)
flix add tmdb movie 19594                        # Bartok the Magnificent (1999)

# Atlantis
flix add tmdb collection 100965                  # Atlantis Collection
flix add tmdb movie 10865                        # Atlantis: The Lost Empire (2001)
flix add tmdb movie 8965                         # Atlantis: Milo's Return (2003)

# Austin Powers
flix add tmdb collection 1006                    # Austin Powers Collection
flix add tmdb movie 816                          # Austin Powers: International Man of Mystery (1997)
flix add tmdb movie 817                          # Austin Powers: The Spy Who Shagged Me (1999)
flix add tmdb movie 818                          # Austin Powers in Goldmember (2002)

# AVP (Aliens vs Predator)
flix add tmdb collection 115762                  # AVP Collection
flix add tmdb movie 395                          # AVP: Alien vs. Predator (2004)
flix add tmdb movie 440                          # Aliens vs Predator: Requiem (2007)

# Avatar
flix add tmdb collection 87096                   # Avatar Collection
flix add tmdb movie 19995                        # Avatar (2009)
flix add tmdb movie 76600                        # Avatar: The Way of Water (2022)
flix add tmdb movie 83533                        # Avatar: Fire and Ash (2025)
# flix add tmdb movie 216527 # upcoming
# flix add tmdb movie 393209 # upcoming

# Avatar: The Last Airbender
flix add tmdb collection 1136366                 # Avatar: The Last Airbender - Collection
flix add tmdb movie 980431                       # Avatar: Aang, The Last Airbender (2026)

# Barbie
flix add flix collection "Barbie Animated Collection" "Collection of the animated films based on Mattel's Barbie."
flix add tmdb movie 15167                        # Barbie in the Nutcracker (2001)
flix add tmdb movie 15015                        # Barbie as Rapunzel (2002)
flix add tmdb movie 15016                        # Barbie of Swan Lake (2003)
flix add --fs-slug "barbie as the princess and the pauper (2004)" --web-slug "barbie-as-the-princess-and-the-pauper-2004" tmdb movie 15165                          # Barbie as The Princess & the Pauper (2004)
flix add tmdb movie 15906                        # Barbie and the Magic of Pegasus (2005)
flix add tmdb movie 13002                        # Barbie in the 12 Dancing Princesses (2006)
flix add tmdb movie 13283                        # Barbie as the Island Princess (2007)
flix add tmdb movie 13004                        # Barbie and the Diamond Castle (2008)
flix add tmdb movie 13459                        # Barbie in 'A Christmas Carol' (2008)
flix add tmdb movie 23566                        # Barbie and the Three Musketeers (2009)
flix add tmdb movie 16418                        # Barbie Presents: Thumbelina (2009)
flix add tmdb collection 411859                  # Barbie Fairytopia Collection
flix add tmdb movie 13284                        # Barbie: Fairytopia (2005)
flix add tmdb movie 13285                        # Barbie Fairytopia: Mermaidia (2006)
flix add tmdb movie 16962                        # Barbie Fairytopia: Magic of the Rainbow (2007)
flix add tmdb collection 447622                  # Barbie Mariposa Collection
flix add tmdb movie 18198                        # Barbie Mariposa (2008)
flix add --fs-slug "barbie mariposa and the fairy princess (2013)" --web-slug "barbie-mariposa-and-the-fairy-princess-2013" tmdb movie 196254                       # Barbie Mariposa & the Fairy Princess (2013)
flix add tmdb movie 21705                        # The Barbie Diaries (2006)

# Beekeeper
flix add tmdb collection 1440784                 # The Beekeeper Collection
flix add tmdb movie 866398                       # The Beekeeper (2024)
# flix add tmdb movie 1440769 # upcoming

# Borat
flix add tmdb collection 747168                  # Borat Moviefilms Collection
flix add tmdb movie 496                          # Borat: Cultural Learnings of America for Make Benefit Glorious Nation of Kazakhstan (2006)
flix add tmdb movie 740985                       # Borat Subsequent Moviefilm: Delivery of Prodigious Bribe to American Regime for Make Benefit Once Glorious Nation of Kazakhstan (2020)

# Captain Underpants
flix add tmdb collection 732397                  # Captain Underpants Collection
flix add tmdb movie 268531                       # Captain Underpants: The First Epic Movie (2017)
flix add tmdb movie 637395                       # The Spooky Tale of Captain Underpants Hack-a-ween (2019)
flix add tmdb movie 668602                       # Captain Underpants Epic Choice-o-Rama (2020)
flix add tmdb movie 768509                       # Captain Underpants Mega Blissmas (2020)

# Cars
flix add tmdb collection 87118                   # Cars Collection
flix add tmdb movie 920                          # Cars (2006)
flix add tmdb movie 49013                        # Cars 2 (2011)
flix add tmdb movie 260514                       # Cars 3 (2017)

# Charlie and Froggy
flix add tmdb collection 700073                  # Charlie Strapp and Froggy Ball Collection
flix add tmdb movie 350086                       # Charlie Strap, Froggy Ball and Their Friends (1956)
flix add tmdb movie 34700                        # Kalle Stropp och Grodan Boll (1987)
flix add tmdb movie 23811                        # Charlie Strap and Froggy Ball Flying High (1991)

# Chickenhare
flix add --title "Chickenhare Collection" tmdb collection 1473079                 # Chickenhare Collection
flix add tmdb movie 778855                       # Chickenhare and the Hamster of Darkness (2022)
flix add tmdb movie 1215532                      # Chickenhare and the Secret of the Groundhog (2025)

# Chicken Run
flix add tmdb collection 718551                  # Chicken Run Collection
flix add tmdb movie 7443                         # Chicken Run (2000)
flix add tmdb movie 520758                       # Chicken Run: Dawn of the Nugget (2023)

# Crank
flix add tmdb collection 64751                   # Crank Collection
flix add tmdb movie 1948                         # Crank (2006)
flix add tmdb movie 15092                        # Crank: High Voltage (2009)

# Death Race
flix add tmdb collection 434533                  # Death Race (1975) Collection
flix add tmdb movie 13282                        # Death Race 2000 (1975)
flix add tmdb movie 401544                       # Death Race 2050 (2017)
flix add tmdb collection 86116                   # Death Race (2008) Collection
flix add tmdb movie 10483                        # Death Race (2008)
flix add tmdb movie 51620                        # Death Race 2 (2010)
flix add tmdb movie 156717                       # Death Race: Inferno (2013)
flix add tmdb movie 401478                       # Death Race: Beyond Anarchy (2018)

# Despicable Me
flix add tmdb collection 86066                   # Despicable Me Collection
flix add tmdb movie 20352                        # Despicable Me (2010)
flix add tmdb movie 93456                        # Despicable Me 2 (2013)
flix add tmdb movie 324852                       # Despicable Me 3 (2017)
flix add tmdb movie 519182                       # Despicable Me 4 (2024)

# The Devil Wears Prada
flix add tmdb collection 1314623                 # The Devil Wears Prada Collection
flix add tmdb movie 350                          # The Devil Wears Prada (2006)
flix add tmdb movie 1314481                      # The Devil Wears Prada 2 (2026)

# Die Hard
flix add tmdb collection 1570                    # Die Hard Collection
flix add tmdb movie 562  # Die Hard (1988)
flix add tmdb movie 1573 # Die Hard 2 (1990)
flix add tmdb movie 1572 # Die Hard: With a Vengeance (1995)
flix add tmdb movie 1571 # Live Free or Die Hard (2007)
flix add tmdb movie 47964 # A Good Day to Die Hard (2013)

# Disney Live-Action Remakes
flix add --fs-slug "disney live-action remakes" --web-slug "disney-live-action-remakes" flix collection "Disney Live-Action Remakes" "Live-action or photorealistic remakes produced by Walt Disney Pictures of its animated films."
flix add tmdb movie 447277                       # The Little Mermaid (2023)
flix add tmdb movie 447273                       # Snow White (2025)

# Divergent
flix add tmdb collection 283579                  # Divergent Collection
flix add tmdb movie 157350                       # Divergent (2014)
flix add tmdb movie 262500                       # Insurgent (2015)
flix add tmdb movie 262504                       # Allegiant (2016)

# Dune
flix add tmdb collection 726871                  # Dune Collection
flix add tmdb movie 438631                       # Dune (2021)
flix add tmdb movie 693134                       # Dune: Part Two (2024)
flix add tmdb movie 1170608                      # Dune: Part Three (2026)

# Emperor's New Groove
flix add tmdb collection 178117                  # The Emperor's New Groove Collection
flix add tmdb movie 11688                        # The Emperor's New Groove (2000)
flix add tmdb movie 13417                        # Kronk's New Groove (2005)

# Enchanted
flix add tmdb collection 531799                  # Enchanted Collection
flix add tmdb movie 4523                         # Enchanted (2007)
flix add tmdb movie 338958                       # Disenchanted (2022)

# Fantastic Beasts
flix add tmdb collection 435259                  # Fantastic Beasts Collection
flix add tmdb movie 259316                       # Fantastic Beasts and Where to Find Them (2016)
flix add tmdb movie 338952                       # Fantastic Beasts: The Crimes of Grindelwald (2018)
flix add tmdb movie 338953                       # Fantastic Beasts: The Secrets of Dumbledore (2022)

# Fast and the Furious
flix add tmdb collection 9485                    # The Fast and the Furious Collection
flix add tmdb movie 9799                         # The Fast and the Furious (2001)
flix add tmdb movie 584                          # 2 Fast 2 Furious (2003)
flix add tmdb movie 9615                         # The Fast and the Furious: Tokyo Drift (2006)
flix add --fs-slug "fast and furious (2009)" --web-slug "fast-and-furious-2009" tmdb movie 13804                                                                        # Fast & Furious (2009)
flix add tmdb movie 51497                        # Fast Five (2011)
flix add --fs-slug "fast and furious 6 (2013)" --web-slug "fast-and-furious-6-2013" tmdb movie 82992                                                                    # Fast & Furious 6 (2013)
flix add tmdb movie 168259                       # Furious 7 (2015)
flix add tmdb movie 337339                       # The Fate of the Furious (2017)
flix add tmdb movie 385128                       # F9 (2021)
flix add tmdb movie 385687                       # Fast X (2023)
flix add --fs-slug "fast and furious presents hobbs and shaw (2019)" --web-slug "fast-and-furious-presents-hobbs-and-shaw-2019" tmdb movie 384018                       # Fast & Furious Presents: Hobbs & Shaw (2019)

# FernGully
flix add tmdb collection 19160                   # FernGully Collection
flix add tmdb movie 13225                        # FernGully: The Last Rainforest (1992)
flix add tmdb movie 18948                        # FernGully 2: The Magical Rescue (1998)

# Fifty Shades
flix add tmdb collection 344830                  # Fifty Shades Collection
flix add tmdb movie 216015                       # Fifty Shades of Grey (2015)
flix add tmdb movie 341174                       # Fifty Shades Darker (2017)
flix add tmdb movie 337167                       # Fifty Shades Freed (2018)

# Finding Nemo
flix add tmdb collection 137697                  # Finding Nemo Collection
flix add tmdb movie 12                           # Finding Nemo (2003)
flix add tmdb movie 127380                       # Finding Dory (2016)

# Frozen
flix add tmdb collection 386382                  # Frozen Collection
flix add tmdb movie 109445                       # Frozen (2013)
flix add tmdb movie 330457                       # Frozen II (2019)
# flix add tmdb movie 1024604 # upcoming
# flix add tmdb movie 1205983 # upcoming

# Garfield Movie
flix add tmdb collection 1510620                 # The Garfield Movie Collection
flix add tmdb movie 748783                       # The Garfield Movie (2024)
# flix add tmdb movie 1510501 # upcoming

# Ghostbusters
flix add tmdb collection 2980                    # Ghostbusters Collection
flix add tmdb movie 620                          # Ghostbusters (1984)
flix add tmdb movie 2978                         # Ghostbusters II (1989)
flix add tmdb movie 43074                        # Ghostbusters (2016)
flix add tmdb movie 425909                       # Ghostbusters: Afterlife (2021)
flix add tmdb movie 967847                       # Ghostbusters: Frozen Empire (2024)

# Gnomeo & Juliet
flix add --fs-slug "gnomeo and juliet collection" --web-slug "gnomeo-and-juliet-collection" tmdb collection 493668                  # Gnomeo & Juliet Collection
flix add --fs-slug "gnomeo and juliet (2011)" --web-slug "gnomeo-and-juliet-2011" tmdb movie 45772                                  # Gnomeo & Juliet (2011)
flix add tmdb movie 370567                       # Sherlock Gnomes (2018)

# Happy Gilmore
flix add tmdb collection 1263259                 # Happy Gilmore Collection
flix add tmdb movie 9614                         # Happy Gilmore (1996)
flix add tmdb movie 1263256                      # Happy Gilmore 2 (2025)

# Harry Potter
flix add tmdb collection 1241                    # Harry Potter Collection
flix add tmdb movie 671                          # Harry Potter and the Philosopher's Stone (2001)
flix add tmdb movie 672                          # Harry Potter and the Chamber of Secrets (2002)
flix add tmdb movie 673                          # Harry Potter and the Prisoner of Azkaban (2004)
flix add tmdb movie 674                          # Harry Potter and the Goblet of Fire (2005)
flix add tmdb movie 675                          # Harry Potter and the Order of the Phoenix (2007)
flix add tmdb movie 767                          # Harry Potter and the Half-Blood Prince (2009)
flix add tmdb movie 12444                        # Harry Potter and the Deathly Hallows: Part 1 (2010)
flix add tmdb movie 12445                        # Harry Potter and the Deathly Hallows: Part 2 (2011)

# Jump Street
flix add tmdb collection 212562                  # Jump Street Collection
flix add tmdb movie 64688                        # 21 Jump Street (2012)
flix add tmdb movie 187017                       # 22 Jump Street (2014)

# Knives Out
flix add tmdb collection 722971                  # Knives Out Collection
flix add tmdb movie 546554                       # Knives Out (2019)
flix add tmdb movie 661374                       # Glass Onion: A Knives Out Mystery (2022)
flix add tmdb movie 812583                       # Wake Up Dead Man: A Knives Out Mystery (2025)

# Home Alone
flix add tmdb collection 9888                    # Home Alone Collection
flix add tmdb movie 771                          # Home Alone (1990)
flix add tmdb movie 772                          # Home Alone 2: Lost in New York (1992)
flix add tmdb movie 9714                         # Home Alone 3 (1997)
flix add tmdb movie 12536                        # Home Alone 4 (2002)

# Hotel Transylvania
flix add tmdb collection 185103                  # Hotel Transylvania Collection
flix add tmdb movie 76492                        # Hotel Transylvania (2012)
flix add tmdb movie 159824                       # Hotel Transylvania 2 (2015)
flix add tmdb movie 400155                       # Hotel Transylvania 3: Summer Vacation (2018)
flix add tmdb movie 585083                       # Hotel Transylvania: Transformania (2022)

# How to Train Your Dragon
flix add tmdb collection 89137                   # How to Train Your Dragon Collection
flix add tmdb movie 10191                        # How to Train Your Dragon (2010)
flix add tmdb movie 82702                        # How to Train Your Dragon 2 (2014)
flix add tmdb movie 166428                       # How to Train Your Dragon: The Hidden World (2019)

# How to Train Your Dragon (Live-Action)
flix add tmdb collection 1458864                 # How to Train Your Dragon (Live-Action) Collection
flix add tmdb movie 1087192                      # How to Train Your Dragon (2025)
flix add tmdb movie 1458319                      # How to Train Your Dragon 2 (2027)

# Hunger Games
flix add tmdb collection 131635                  # The Hunger Games Collection
flix add tmdb movie 70160                        # The Hunger Games (2012)
flix add tmdb movie 101299                       # The Hunger Games: Catching Fire (2013)
flix add tmdb movie 131631                       # The Hunger Games: Mockingjay - Part 1 (2014)
flix add tmdb movie 131634                       # The Hunger Games: Mockingjay - Part 2 (2015)

# Hunger Games - New Trilogy
flix add --title "The Hunger Games - New Trilogy Collection" --fs-slug "hunger games new trilogy collection" --web-slug "hunger-games-new-trilogy-collection" tmdb collection 1701563
flix add --fs-slug "hunger games the ballad of songbirds and snakes (2023)" --web-slug "hunger-games-the-ballad-of-songbirds-and-snakes-2023" tmdb movie 695721                       # The Hunger Games: The Ballad of Songbirds & Snakes (2023)
# flix add tmdb movie 1300968 # upcoming

# Huntsman Collection
flix add tmdb collection 393379                  # The Huntsman Collection
flix add tmdb movie 58595                        # Snow White and the Huntsman (2012)
flix add tmdb movie 290595                       # The Huntsman: Winter's War (2016)

# Incredibles
flix add tmdb collection 468222                  # The Incredibles Collection
flix add tmdb movie 9806                         # The Incredibles (2004)
flix add tmdb movie 260513                       # Incredibles 2 (2018)
# flix add tmdb movie 1327821 # upcoming

# Inside Out
flix add tmdb collection 1022790                 # Inside Out Collection
flix add tmdb movie 150540                       # Inside Out (2015)
flix add tmdb movie 1022789                      # Inside Out 2 (2024)

# Jack Reacher
flix add tmdb collection 403374                  # Jack Reacher Collection
flix add tmdb movie 75780                        # Jack Reacher (2012)
flix add tmdb movie 343611                       # Jack Reacher: Never Go Back (2016)

# Jason Bourne
flix add tmdb collection 31562                   # The Bourne Collection
flix add tmdb movie 2501                         # The Bourne Identity (2002)
flix add tmdb movie 2502                         # The Bourne Supremacy (2004)
flix add tmdb movie 2503                         # The Bourne Ultimatum (2007)
flix add tmdb movie 49040                        # The Bourne Legacy (2012)
flix add tmdb movie 324668                       # Jason Bourne (2016)

# John Wick
flix add tmdb collection 404609                  # John Wick Collection
flix add tmdb movie 245891                       # John Wick (2014)
flix add tmdb movie 324552                       # John Wick: Chapter 2 (2017)
flix add --web-slug "john-wick-chapter-3-parabellum-2019" tmdb movie 458156                       # John Wick: Chapter 3 - Parabellum (2019)
flix add tmdb movie 603692                       # John Wick: Chapter 4 (2023)

# Johnny English
flix add tmdb collection 86117                   # Johnny English Collection
flix add tmdb movie 9486                         # Johnny English (2003)
flix add tmdb movie 58233                        # Johnny English Reborn (2011)
flix add tmdb movie 463272                       # Johnny English Strikes Again (2018)
# flix add tmdb movie 1241132 # upcoming

# Jumanji
flix add tmdb collection 495527                  # Jumanji Collection
flix add tmdb movie 8844                         # Jumanji (1995)
flix add tmdb movie 353486                       # Jumanji: Welcome to the Jungle (2017)
flix add tmdb movie 512200                       # Jumanji: The Next Level (2019)
# flix add tmdb movie 1260649 # upcoming

# Kung Fu Panda
flix add tmdb collection 77816                   # Kung Fu Panda Collection
flix add tmdb movie 9502                         # Kung Fu Panda (2008)
flix add tmdb movie 49444                        # Kung Fu Panda 2 (2011)
flix add tmdb movie 140300                       # Kung Fu Panda 3 (2016)
flix add tmdb movie 1011985                      # Kung Fu Panda 4 (2024)

# Last Witch Hunter
flix add tmdb collection 1543569                 # The Last Witch Hunter Collection
flix add tmdb movie 274854                       # The Last Witch Hunter (2015)
# flix add tmdb movie 1543515 # upcoming

# Lego Movie
flix add tmdb collection 325470                  # The Lego Movie Collection
flix add tmdb movie 137106                       # The Lego Movie (2014)
flix add tmdb movie 280217                       # The Lego Movie 2: The Second Part (2019)

# Leo
flix add tmdb collection 1235977                 # Leo Collection
flix add tmdb movie 1075794                      # Leo (2023)
# flix add tmdb movie 1235976 # upcoming

# Lilo & Stitch
flix add --fs-slug "lilo and stitch animated collection" --web-slug "lilo-and-stitch-animated-collection" tmdb collection 97461                                   # Lilo & Stitch (Animated) Collection
flix add --fs-slug "lilo and stitch (2002)" --web-slug "lilo-and-stitch-2002" tmdb movie 11544                                                                    # Lilo & Stitch (2002)
flix add tmdb movie 15567                        # Stitch! The Movie (2003)
flix add --fs-slug "lilo and stitch 2 stitch has a glitch (2005)" --web-slug "lilo-and-stitch-2-stitch-has-a-glitch-2005" tmdb movie 20760                        # Lilo & Stitch 2: Stitch Has a Glitch (2005)
flix add --fs-slug "leroy and stitch (2006)" --web-slug "leroy-and-stitch-2006" tmdb movie 21316                                                                  # Leroy & Stitch (2006)

# Lion King
flix add tmdb collection 94032                   # The Lion King Collection
flix add tmdb movie 8587                         # The Lion King (1994)
flix add tmdb movie 9732                         # The Lion King II: Simba's Pride (1998)
# flix add tmdb movie 11430 # unicode

# Mad Max
flix add tmdb collection 8945                    # Mad Max Collection
flix add tmdb movie 9659                         # Mad Max (1979)
flix add tmdb movie 8810                         # Mad Max 2 (1981)
flix add tmdb movie 9355                         # Mad Max Beyond Thunderdome (1985)
flix add tmdb movie 76341                        # Mad Max: Fury Road (2015)
flix add tmdb movie 786892                       # Furiosa: A Mad Max Saga (2024)
# flix add tmdb movie 340023 # upcoming

# Madagascar
flix add tmdb collection 14740                   # Madagascar Collection
flix add tmdb movie 953                          # Madagascar (2005)
flix add tmdb movie 10527                        # Madagascar: Escape 2 Africa (2008)
flix add tmdb movie 80321                        # Madagascar 3: Europe's Most Wanted (2012)

# Matrix
flix add tmdb collection 2344                    # The Matrix Collection
flix add tmdb movie 603                          # The Matrix (1999)
flix add tmdb movie 604                          # The Matrix Reloaded (2003)
flix add tmdb movie 605                          # The Matrix Revolutions (2003)
flix add tmdb movie 624860                       # The Matrix Resurrections (2021)

# Mean Girls
flix add tmdb collection 99606                   # Mean Girls Collection
flix add tmdb movie 10625                        # Mean Girls (2004)
flix add tmdb movie 51481                        # Mean Girls 2 (2011)

# Mechanic
flix add tmdb collection 280504                  # Mechanic Collection
flix add tmdb movie 27582                        # The Mechanic (2011)
flix add tmdb movie 278924                       # Mechanic: Resurrection (2016)

# Meg
flix add tmdb collection 742536                  # The Meg Collection
flix add tmdb movie 345940                       # The Meg (2018)
flix add tmdb movie 615656                       # Meg 2: The Trench (2023)

# Megamind
flix add tmdb collection 1239518                 # Megamind Collection
flix add tmdb movie 38055                        # Megamind (2010)
flix add tmdb movie 1239251                      # Megamind vs. the Doom Syndicate (2024)

# Men in Black
flix add tmdb collection 86055                   # Men in Black Collection
flix add tmdb movie 607                          # Men in Black (1997)
flix add tmdb movie 608                          # Men in Black II (2002)
flix add tmdb movie 41154                        # Men in Black 3 (2012)
flix add tmdb movie 479455                       # Men in Black: International (2019)

# Middle-Earth
flix add flix collection "Middle-Earth Collection" "Peter Jackson's film trilogies (Lord of the Rings and The Hobbit) taking place in Tolkien's Middle-Earth"
flix add tmdb collection 121938                  # The Hobbit Collection
flix add tmdb movie 49051                        # The Hobbit: An Unexpected Journey (2012)
flix add tmdb movie 57158                        # The Hobbit: The Desolation of Smaug (2013)
flix add tmdb movie 122917                       # The Hobbit: The Battle of the Five Armies (2014)
flix add tmdb collection 119                     # The Lord of the Rings Collection
flix add tmdb movie 120                          # The Lord of the Rings: The Fellowship of the Ring (2001)
flix add tmdb movie 121                          # The Lord of the Rings: The Two Towers (2002)
flix add tmdb movie 122                          # The Lord of the Rings: The Return of the King (2003)

# Minecraft
flix add tmdb collection 1461530                 # The Minecraft Movie Collection
flix add tmdb movie 950387                       # A Minecraft Movie (2025)

# Minions
flix add tmdb collection 544669                  # Minions Collection
flix add tmdb movie 211672                       # Minions (2015)
flix add tmdb movie 438148                       # Minions: The Rise of Gru (2022)
# flix add tmdb movie 1315772 # upcoming

# Mission Impossible
flix add tmdb collection 87359                   # Mission: Impossible Collection
flix add tmdb movie 954                          # Mission: Impossible (1996)
flix add tmdb movie 955                          # Mission: Impossible II (2000)
flix add tmdb movie 956                          # Mission: Impossible III (2006)
flix add --web-slug "mission-impossible-ghost-protocol-2011" tmdb movie 56292                                 # Mission: Impossible - Ghost Protocol (2011)
flix add --web-slug "mission-impossible-rogue-nation-2015" tmdb movie 177677                                  # Mission: Impossible - Rogue Nation (2015)
flix add --web-slug "mission-impossible-fallout-2018" tmdb movie 353081                                       # Mission: Impossible - Fallout (2018)
flix add --web-slug "mission-impossible-dead-reckoning-part-one-2023" tmdb movie 575264                       # Mission: Impossible - Dead Reckoning Part One (2023)
flix add --web-slug "mission-impossible-the-final-reckoning-2025" tmdb movie 575265                           # Mission: Impossible - The Final Reckoning (2025)

# Moana
flix add tmdb collection 1241984                 # Moana Collection
flix add tmdb movie 277834                       # Moana (2016)
flix add tmdb movie 1241982                      # Moana 2 (2024)

# Monsters Inc
flix add tmdb collection 137696                  # Monsters, Inc. Collection
flix add tmdb movie 585                          # Monsters, Inc. (2001)
flix add tmdb movie 62211                        # Monsters University (2013)

# Mr. Bean
flix add tmdb collection 103372                  # Mr. Bean Collection
flix add tmdb movie 1281                         # Bean (1997)
flix add tmdb movie 1268                         # Mr. Bean's Holiday (2007)

# Mummy
flix add tmdb collection 1733                    # The Mummy Collection
flix add tmdb movie 564                          # The Mummy (1999)
flix add tmdb movie 1734                         # The Mummy Returns (2001)
flix add tmdb movie 1735                         # The Mummy: Tomb of the Dragon Emperor (2008)
# flix add tmdb movie 1574840 # upcoming

# My Spy
flix add tmdb collection 1090373                 # My Spy Collection
flix add tmdb movie 592834                       # My Spy (2020)
flix add tmdb movie 1048241                      # My Spy The Eternal City (2024)

# Now You See Me
flix add tmdb collection 382685                  # Now You See Me Collection
flix add tmdb movie 75656                        # Now You See Me (2013)
flix add tmdb movie 291805                       # Now You See Me 2 (2016)
flix add tmdb movie 425274                       # Now You See Me: Now You Don't (2025)
# flix add tmdb movie 1457735 # upcoming

# Ocean's
flix add tmdb collection 304                     # Ocean's Collection
flix add tmdb movie 161                          # Ocean's Eleven (2001)
flix add tmdb movie 163                          # Ocean's Twelve (2004)
flix add tmdb movie 298                          # Ocean's Thirteen (2007)
flix add tmdb movie 402900                       # Ocean's Eight (2018)

# Pacific Rim
flix add tmdb collection 363369                  # Pacific Rim Collection
flix add tmdb movie 68726                        # Pacific Rim (2013)
flix add tmdb movie 268896                       # Pacific Rim: Uprising (2018)

# Percy Jackson
flix add tmdb collection 179919                  # Percy Jackson Collection
flix add --fs-slug "percy jackson and the olympians the lightning thief (2010)" --web-slug "percy-jackson-and-the-olympians-the-lightning-thief-2010" tmdb movie 32657                        # Percy Jackson & the Olympians: The Lightning Thief (2010)
flix add tmdb movie 76285                        # Percy Jackson: Sea of Monsters (2013)

# Pink Panther (Steve Martin)
flix add tmdb collection 130440                  # The Pink Panther (Steve Martin) Collection
flix add tmdb movie 12096                        # The Pink Panther (2006)
flix add tmdb movie 15159                        # The Pink Panther 2 (2009)

# Pirates of the Caribbean
flix add tmdb collection 295                     # Pirates of the Caribbean Collection
flix add tmdb movie 22                           # Pirates of the Caribbean: The Curse of the Black Pearl (2003)
flix add tmdb movie 58                           # Pirates of the Caribbean: Dead Man's Chest (2006)
flix add tmdb movie 285                          # Pirates of the Caribbean: At World's End (2007)
flix add tmdb movie 1865                         # Pirates of the Caribbean: On Stranger Tides (2011)
flix add tmdb movie 166426                       # Pirates of the Caribbean: Dead Men Tell No Tales (2017)

# Quiet Place
flix add tmdb collection 521226                  # A Quiet Place Collection
flix add tmdb movie 447332                       # A Quiet Place (2018)
flix add tmdb movie 520763                       # A Quiet Place Part II (2021)
# flix add tmdb movie 1324923 # upcoming
flix add tmdb movie 762441                       # A Quiet Place: Day One (2024)

# Resident Evil
flix add tmdb collection 17255                   # Resident Evil Collection
flix add tmdb movie 1576                         # Resident Evil (2002)
flix add tmdb movie 1577                         # Resident Evil: Apocalypse (2004)
flix add tmdb movie 7737                         # Resident Evil: Extinction (2007)
flix add tmdb movie 35791                        # Resident Evil: Afterlife (2010)
flix add tmdb movie 71679                        # Resident Evil: Retribution (2012)
flix add tmdb movie 173897                       # Resident Evil: The Final Chapter (2016)
flix add tmdb movie 460458                       # Resident Evil: Welcome to Raccoon City (2021)

# Ring
flix add tmdb collection 14563                   # The Ring Collection
flix add tmdb movie 565                          # The Ring (2002)
flix add tmdb movie 10320                        # The Ring Two (2005)
flix add tmdb movie 14564                        # Rings (2017)

# RIPD
flix add tmdb collection 1041695                 # R.I.P.D. Collection
flix add tmdb movie 49524                        # R.I.P.D. (2013)
flix add tmdb movie 1013860                      # R.I.P.D. 2: Rise of the Damned (2022)

# Scary Movie
flix add tmdb collection 4246                    # Scary Movie Collection
flix add tmdb movie 4247                         # Scary Movie (2000)
flix add tmdb movie 4248                         # Scary Movie 2 (2001)
flix add tmdb movie 4256                         # Scary Movie 3 (2003)
flix add tmdb movie 4257                         # Scary Movie 4 (2006)
flix add tmdb movie 4258                         # Scary Movie 5 (2013)
# flix add tmdb movie 1273221 # upcoming

# Scooby-Doo
flix add tmdb collection 86860                   # Scooby-Doo Collection
flix add tmdb movie 9637                         # Scooby-Doo (2002)
flix add tmdb movie 11024                        # Scooby-Doo 2: Monsters Unleashed (2004)

# Secret Life of Pets
flix add tmdb collection 427084                  # The Secret Life of Pets Collection
flix add tmdb movie 328111                       # The Secret Life of Pets (2016)
flix add tmdb movie 412117                       # The Secret Life of Pets 2 (2019)
# flix add tmdb movie 1109206 # upcoming

# Secret of NIMH
flix add tmdb collection 141292                  # The Secret of NIMH Collection
flix add tmdb movie 11704                        # The Secret of NIMH (1982)
flix add tmdb movie 20756                        # The Secret of NIMH 2: Timmy to the Rescue (1999)

# Sharkboy and Lavagirl
flix add tmdb collection 727089                  # Sharkboy and Lavagirl Collection
flix add tmdb movie 14199                        # The Adventures of Sharkboy and Lavagirl (2005)
flix add tmdb movie 615677                       # We Can Be Heroes (2020)
# flix add tmdb movie 783103 # upcoming

# Shrek
flix add tmdb collection 2150                    # Shrek Collection
flix add tmdb movie 808                          # Shrek (2001)
flix add tmdb movie 809                          # Shrek 2 (2004)
flix add tmdb movie 810                          # Shrek the Third (2007)
flix add tmdb movie 10192                        # Shrek Forever After (2010)
# flix add tmdb movie 421892 # upcoming

# Sing
flix add tmdb collection 544670                  # Sing Collection
flix add tmdb movie 335797                       # Sing (2016)
flix add tmdb movie 438695                       # Sing 2 (2021)
# flix add tmdb movie 1108794 # upcoming

# Sonic
flix add tmdb collection 720879                  # Sonic the Hedgehog Collection
flix add tmdb movie 454626                       # Sonic the Hedgehog (2020)
flix add tmdb movie 675353                       # Sonic the Hedgehog 2 (2022)
flix add tmdb movie 939243                       # Sonic the Hedgehog 3 (2024)
# flix add tmdb movie 1401586 # upcoming

# Space Jam
flix add tmdb collection 722961                  # Space Jam Collection
flix add tmdb movie 2300                         # Space Jam (1996)
flix add tmdb movie 379686                       # Space Jam: A New Legacy (2021)

# Spaceballs
flix add tmdb collection 1497153                 # Spaceballs Collection
flix add tmdb movie 957                          # Spaceballs (1987)
# flix add tmdb movie 1306322 # upcoming

# Spy Kids
flix add tmdb collection 86486                   # Spy Kids Collection
flix add tmdb movie 10054                        # Spy Kids (2001)
flix add tmdb movie 9488                         # Spy Kids 2: The Island of Lost Dreams (2002)
flix add --fs-slug "spy kids 3d game over (2003)" --web-slug "spy-kids-3d-game-over-2003" tmdb movie 12279                        # Spy Kids 3-D: Game Over (2003)
flix add tmdb movie 56288                        # Spy Kids: All the Time in the World (2011)

# Super Mario
flix add tmdb collection 1434561                 # The Super Mario Collection
flix add tmdb movie 502356                       # The Super Mario Bros. Movie (2023)
flix add tmdb movie 1226863                      # The Super Mario Galaxy Movie (2026)

# Taken
flix add tmdb collection 135483                  # Taken Collection
flix add tmdb movie 8681                         # Taken (2008)
flix add tmdb movie 82675                        # Taken 2 (2012)
flix add tmdb movie 260346                       # Taken 3 (2014)

# Terminator
flix add tmdb collection 528                     # The Terminator Collection
flix add tmdb movie 218                          # The Terminator (1984)
flix add tmdb movie 280                          # Terminator 2: Judgment Day (1991)
flix add tmdb movie 296                          # Terminator 3: Rise of the Machines (2003)
flix add tmdb movie 534                          # Terminator Salvation (2009)
flix add tmdb movie 87101                        # Terminator Genisys (2015)
flix add tmdb movie 290859                       # Terminator: Dark Fate (2019)

# Tomb Raider
flix add tmdb collection 2467                    # Tomb Raider Collection
flix add tmdb movie 1995                         # Lara Croft: Tomb Raider (2001)
flix add tmdb movie 1996                         # Lara Croft: Tomb Raider - The Cradle of Life (2003)
flix add tmdb movie 338970                       # Tomb Raider (2018)

# Top Gun
flix add tmdb collection 531330                  # Top Gun Collection
flix add tmdb movie 744                          # Top Gun (1986)
flix add tmdb movie 361743                       # Top Gun: Maverick (2022)
# flix add tmdb movie 1229934 # upcoming

# Toy Story
flix add tmdb collection 10194                   # Toy Story Collection
flix add tmdb movie 862                          # Toy Story (1995)
flix add tmdb movie 863                          # Toy Story 2 (1999)
flix add tmdb movie 10193                        # Toy Story 3 (2010)
flix add tmdb movie 301528                       # Toy Story 4 (2019)
# flix add tmdb movie 1084244                    # upcoming

# Transporter
flix add tmdb collection 9518                    # The Transporter Collection
flix add tmdb movie 4108                         # The Transporter (2002)
flix add tmdb movie 9335                         # Transporter 2 (2005)
flix add tmdb movie 13387                        # Transporter 3 (2008)
flix add tmdb movie 287948                       # The Transporter Refueled (2015)

# Transformers
flix add tmdb collection 8650                    # Transformers Collection
flix add tmdb movie 1858                         # Transformers (2007)
flix add tmdb movie 8373                         # Transformers: Revenge of the Fallen (2009)
flix add tmdb movie 38356                        # Transformers: Dark of the Moon (2011)
flix add tmdb movie 91314                        # Transformers: Age of Extinction (2014)
flix add tmdb movie 335988                       # Transformers: The Last Knight (2017)

# Trolls
flix add tmdb collection 489724                  # The Trolls Collection
flix add tmdb movie 136799                       # Trolls (2016)
flix add tmdb movie 446893                       # Trolls World Tour (2020)
flix add tmdb movie 901362                       # Trolls Band Together (2023)

# TRON
flix add tmdb collection 63043                   # TRON Collection
flix add tmdb movie 97                           # Tron (1982)
flix add tmdb movie 20526                        # TRON: Legacy (2010)
flix add tmdb movie 533533                       # TRON: Ares (2025)

# Twilight
flix add tmdb collection 33514                   # The Twilight Collection
flix add tmdb movie 8966                         # Twilight (2008)
flix add tmdb movie 18239                        # The Twilight Saga: New Moon (2009)
flix add tmdb movie 24021                        # The Twilight Saga: Eclipse (2010)
flix add --web-slug "twilight-saga-breaking-dawn-part-1-2011" tmdb movie 50619                        # The Twilight Saga: Breaking Dawn - Part 1 (2011)
flix add --web-slug "twilight-saga-breaking-dawn-part-2-2012" tmdb movie 50620                        # The Twilight Saga: Breaking Dawn - Part 2 (2012)

# Underworld
flix add tmdb collection 2326                    # Underworld Collection
flix add tmdb movie 277                          # Underworld (2003)
flix add tmdb movie 834                          # Underworld: Evolution (2006)
flix add tmdb movie 12437                        # Underworld: Rise of the Lycans (2009)
flix add tmdb movie 52520                        # Underworld: Awakening (2012)
flix add tmdb movie 346672                       # Underworld: Blood Wars (2016)

# Up
flix add flix collection "Up Collection" ""
flix add tmdb movie 14160                        # Up (2009)
flix add tmdb movie 1076364                      # Carl's Date (2023)

# Vampires Suck
flix add tmdb collection 278920                  # Vampires Suck Collection
flix add tmdb movie 40264                        # Vampires Suck (2010)
flix add tmdb movie 80384                        # Breaking Wind (2012)

# Violent Night
flix add tmdb collection 1078456                 # Violent Night Collection
flix add tmdb movie 899112                       # Violent Night (2022)
# flix add tmdb movie 1078446 # upcoming

# Wild Robot
flix add tmdb collection 1370345                 # The Wild Robot Collection
flix add tmdb movie 1184918                      # The Wild Robot (2024)
# flix add tmdb movie 1370190 # upcoming

# Wonka
flix add tmdb collection 1393821                 # Wonka Collection
flix add tmdb movie 787699                       # Wonka (2023)
# flix add tmdb movie 1385209 # upcoming

# Zombieland
flix add tmdb collection 537982                  # Zombieland Collection
flix add tmdb movie 19908                        # Zombieland (2009)
flix add tmdb movie 338967                       # Zombieland: Double Tap (2019)

# Zootopia
flix add tmdb collection 1084247                 # Zootopia Collection
flix add tmdb movie 269149                       # Zootopia (2016)
flix add tmdb movie 1084242                      # Zootopia 2 (2025)


####################
# Show Collections #
####################

# Arrowverse
flix add flix collection "Arrowverse" "A television franchise that is based on characters that appear in publications by DC Comics."
flix add tmdb show 1412                          # Arrow (2012)
flix add tmdb show 89247                         # Batwoman (2019)
flix add tmdb show 71663                         # Black Lightning (2018)
flix add tmdb show 60735                         # The Flash (2014)
flix add --fs-slug "legends of tomorrow (2016)" --web-slug "legends-of-tomorrow-2016" tmdb show 62643
flix add tmdb show 62688                         # Supergirl (2015)

# Avatar Universe
flix add flix collection "Avatar Universe" "Avatar: The Last Airbender is set in an Asiatic-like world in which some people can manipulate the classical elements with psychokinetic variants of the Chinese martial arts known as 'bending'."
flix add tmdb show 246                           # Avatar: The Last Airbender (2005)
flix add tmdb show 33880                         # The Legend of Korra (2012)

# Breaking Bad
flix add flix collection "Breaking Bad Collection" "Collection containing the original Breaking Bad show along with a documentary chronicling the process of making the final season, a sequel movie and prequel show."
flix add tmdb show 1396                          # Breaking Bad (2008)

# Buffyverse
flix add flix collection "Buffyverse" "The Buffyverse is a setting in which supernatural phenomena exist, and supernatural evil can be challenged by people willing to fight against such forces."
flix add tmdb show 2426                          # Angel (1999)
flix add tmdb show 95                            # Buffy the Vampire Slayer (1997)


##########
# Movies #
##########
flix add tmdb movie 16996                        # 17 Again (2009)
flix add tmdb movie 6957                         # The 40 Year Old Virgin (2005)
flix add tmdb movie 665251                       # Altered Carbon: Resleeved (2020)
flix add tmdb movie 1039773                      # The American Society of Magical Negroes (2024)
flix add tmdb movie 8916                         # Antz (1998)
flix add tmdb movie 1087891                      # The Amateur (2025)
flix add tmdb movie 1072790                      # Anyone But You (2023)
flix add tmdb movie 339403                       # Baby Driver (2017)
flix add tmdb movie 8848                         # The Bank Job (2008)
flix add tmdb movie 5559                         # Bee Movie (2007)
flix add tmdb movie 401123                       # Beta Test (2016)
flix add tmdb movie 55846                        # Blitz (2011)
flix add tmdb movie 651571                       # Breach (2020)
flix add --sort-title "bruno" --fs-slug "bruno (2009)" --web-slug "bruno-2009" tmdb movie 18480                               # Brüno (2009)
flix add tmdb movie 9487                         # A Bug's Life (1998)
flix add tmdb movie 9759                         # Cellular (2004)
flix add tmdb movie 937287                       # Challengers (2024)
flix add tmdb movie 5289                         # Chaos (2005)
flix add tmdb movie 118                          # Charlie and the Chocolate Factory (2005)
flix add tmdb movie 9982                         # Chicken Little (2005)
flix add tmdb movie 15028                        # Clockstoppers (2002)
flix add tmdb movie 1538                         # Collateral (2004)
flix add tmdb movie 9612                         # Coneheads (1993)
flix add --web-slug "constantine-city-of-demons-the-movie-2018" tmdb movie 539517                                             # Constantine: City of Demons - The Movie (2018)
flix add tmdb movie 241843                       # Cooties (2014)
flix add tmdb movie 763215                       # Damsel (2024)
flix add tmdb movie 71469                        # The Darkest Hour (2011)
flix add tmdb movie 76493                        # The Dictator (2012)
flix add tmdb movie 1097870                      # Dear Santa (2024)
flix add tmdb movie 351460                       # Death Note (2017)
flix add tmdb movie 19901                        # Daybreakers (2010)
flix add tmdb movie 10398                        # Double Jeopardy (1999)
flix add tmdb movie 37735                        # Easy A (2010)
flix add tmdb movie 777443                       # The Electric State (2025)
flix add tmdb movie 976573                       # Elemental (2023)
flix add tmdb movie 10719                        # Elf (2003)
flix add tmdb movie 1022787                      # Elio (2025)
flix add tmdb movie 7299                         # Equilibrium (2002)
flix add tmdb movie 746036                       # The Fall Guy (2024)
flix add tmdb movie 11619                        # Flushed Away (2006)
flix add tmdb movie 550988                       # Free Guy (2021)
flix add tmdb movie 228150                       # Fury (2014)
flix add tmdb movie 11665                        # Get Smart (2008)
flix add tmdb movie 10016                        # Ghosts of Mars (2001)
flix add tmdb movie 1297842                      # GOAT (2026)
flix add tmdb movie 950396                       # The Gorge (2025)
flix add tmdb movie 360920                       # The Grinch (2018)
flix add tmdb movie 325348                       # Hardcore Henry (2015)
flix add tmdb movie 826510                       # Harold and the Purple Crayon (2024)
flix add tmdb movie 756397                       # Hitpig! (2024)
flix add tmdb movie 228161                       # Home (2015)
flix add tmdb movie 204082                       # Homefront (2013)
flix add tmdb movie 1327819                      # Hoppers (2026)
flix add tmdb movie 136418                       # Hummingbird (2013)
flix add tmdb movie 7512                         # Idiocracy (2006)
flix add tmdb movie 639720                       # IF (2024)
flix add tmdb movie 49530                        # In Time (2011)
flix add tmdb movie 27205                        # Inception (2010)
flix add tmdb movie 228967                       # The Interview (2014)
flix add tmdb movie 9654                         # The Italian Job (2003)
flix add tmdb movie 1079091                      # It Ends with Us (2024)
flix add tmdb movie 12589                        # Jimmy Neutron: Boy Genius (2001)
flix add tmdb movie 9886                         # Johnny Mnemonic (1995)
flix add tmdb movie 8247                         # Jumper (2008)
flix add tmdb movie 451048                       # Jungle Cruise (2021)
flix add tmdb movie 76757                        # Jupiter Ascending (2015)
flix add tmdb movie 49021                        # Killer Elite (2011)
flix add tmdb movie 1210646                      # Lady Ballers (2023)
flix add tmdb movie 586954                       # Latte and the Magic Waterstone (2019)
flix add tmdb movie 726209                       # Leave the World Behind (2023)
flix add tmdb movie 22894                        # Legion (2010)
flix add tmdb movie 51876                        # Limitless (2011)
flix add tmdb movie 993784                       # Lisa Frankenstein (2024)
flix add tmdb movie 73723                        # The Lorax (2012)
flix add tmdb movie 7515                         # London (2005)
flix add tmdb movie 59967                        # Looper (2012)
flix add tmdb movie 508943                       # Luca (2021)
flix add tmdb movie 240832                       # Lucy (2014)
flix add tmdb movie 286217                       # The Martian (2015)
flix add tmdb movie 13908                        # The Master of Disguise (2002)
flix add tmdb movie 673593                       # Mean Girls (2024)
flix add tmdb movie 593643                       # The Menu (2022)
flix add tmdb movie 940551                       # Migration (2023)
flix add tmdb movie 188161                       # A Million Ways to Die in the West (2014)
flix add tmdb movie 180                          # Minority Report (2002)
flix add tmdb movie 9297                         # Monster House (2006)
flix add tmdb movie 458576                       # Monster Hunter (2020)
flix add tmdb movie 15512                        # Monsters vs Aliens (2009)
flix add tmdb movie 428078                       # Mortal Engines (2018)
flix add --fs-slug "mr and mrs smith (2005)" --web-slug "mr-and-mrs-smith-2005" tmdb movie 787                                # Mr. & Mrs. Smith (2005)
flix add tmdb movie 9824                         # Mystery Men (1999)
flix add tmdb movie 242582                       # Nightcrawler (2014)
flix add tmdb movie 179826                       # Odd Thomas (2013)
flix add tmdb movie 10796                        # The One (2001)
flix add tmdb movie 51828                        # One Day (2011)
flix add tmdb movie 872585                       # Oppenheimer (2023)
flix add tmdb movie 1139829                      # Orion and the Dark (2024)
flix add tmdb movie 7518                         # Over the Hedge (2006)
flix add tmdb movie 10022                        # The Pacifier (2005)
flix add tmdb movie 119283                       # Parker (2013)
flix add --fs-slug "peter pan and wendy (2023)" --web-slug "peter-pan-and-wendy-2023" tmdb movie 420808                       # Peter Pan & Wendy (2023)
flix add tmdb movie 1146058                      # The Pout-Pout Fish (2026)
flix add tmdb movie 38321                        # Priest (2011)
flix add tmdb movie 9543                         # Prince of Persia: The Sands of Time (2010)
flix add tmdb movie 176241                       # Prison Break: The Final Break (2009)
flix add tmdb movie 523638                       # Prisoners of the Ghostland (2021)
flix add tmdb movie 427641                       # Rampage (2018)
flix add tmdb movie 44896                        # Rango (2011)
flix add tmdb movie 2062                         # Ratatouille (2007)
flix add tmdb movie 333339                       # Ready Player One (2018)
flix add tmdb movie 31867                        # Repo Men (2010)
flix add tmdb movie 10851                        # Revolver (2005)
flix add tmdb movie 1022690                      # Ricky Stanicky (2024)
flix add tmdb movie 9346                         # Risky Business (1983)
flix add tmdb movie 72387                        # Safe (2012)
flix add tmdb movie 27576                        # Salt (2010)
flix add tmdb movie 254128                       # San Andreas (2015)
flix add tmdb movie 223702                       # Sausage Party (2016)
flix add tmdb movie 11459                        # Sky High (2005)
flix add tmdb movie 1084153                      # Sneaks (2025)
flix add tmdb movie 110415                       # Snowpiercer (2013)
flix add tmdb movie 27022                        # The Sorcerer's Apprentice (2010)
flix add tmdb movie 592983                       # Spellbound (2024)
flix add tmdb movie 238713                       # Spy (2015)
flix add tmdb movie 53565                        # Steamboat Willie (1928)
flix add tmdb movie 12133                        # Step Brothers (2008)
flix add tmdb movie 9978                         # Stormbreaker (2006)
flix add tmdb movie 11918                        # Superhero Movie (2008)
flix add tmdb movie 9607                         # Super Mario Bros. (1993)
flix add tmdb movie 1007757                      # Swapped (2026)
flix add --title "Taylor Swift | The Eras Tour" tmdb movie 1160164                      # Taylor Swift | The Eras Tour (2023)
flix add tmdb movie 597                          # Titanic (1997)
flix add tmdb movie 158852                       # Tomorrowland (2015)
flix add tmdb movie 157353                       # Transcendence (2014)
flix add tmdb movie 1923                         # Twin Peaks: Fire Walk with Me (1992)
flix add tmdb movie 473033                       # Uncut Gems (2019)
flix add tmdb movie 609271                       # Under the Boardwalk (2023)
flix add tmdb movie 748167                       # Uglies (2024)
flix add tmdb movie 339964                       # Valerian and the City of a Thousand Planets (2017)
flix add tmdb movie 177494                       # Veronica Mars (2014)
flix add --sort-title "wall-e" --fs-slug "wall-e (2008)" --web-slug "wall-e-2008" tmdb movie 10681                            # WALL·E (2008)
flix add tmdb movie 8909                         # Wanted (2008)
flix add tmdb movie 10431                        # War (2007)
flix add tmdb movie 894205                       # Werewolf by Night (2022)
flix add tmdb movie 957258                       # What Is a Woman? (2022)
flix add tmdb movie 265208                       # Wild Card (2015)
flix add tmdb movie 1022796                      # Wish (2023)
flix add tmdb movie 637649                       # Wrath of Man (2021)
flix add tmdb movie 6795                         # Zathura: A Space Adventure (2005)

#########
# Shows #
#########
flix add tmdb show 68421                         # Altered Carbon (2018)
flix add tmdb show 62110                         # Assassination Classroom (2015)
flix add tmdb show 1429                          # Attack on Titan (2013)
flix add tmdb show 42009                         # Black Mirror (2011)
flix add tmdb show 1911                          # Bones (2005)
flix add tmdb show 48891                         # Brooklyn Nine-Nine (2013)
flix add flix episode "brooklyn-nine-nine-2013" 8 10 "The Last Day (Part 2)" "The squad takes stock of its eight years together and looks toward the future." "2021-09-16"
flix add tmdb show 3787                          # Chaotic (2006)
flix add tmdb show 2557                          # Class of the Titans (2006)
flix add tmdb show 13916                         # Death Note (2006)
flix add tmdb show 1405                          # Dexter (2006)
flix add tmdb show 1399                          # Game of Thrones (2011)
flix add tmdb show 40075                         # Gravity Falls (2012)
flix add tmdb show 1639                          # Heroes (2006)
flix add tmdb show 60858                         # Heroes Reborn (2015)
flix add tmdb show 62687                         # Limitless (2015)
flix add tmdb show 60846                         # Log Horizon (2013)
flix add tmdb show 64432                         # The Magicians (2015)
flix add tmdb show 5920                          # The Mentalist (2008)
flix add tmdb show 12786                         # Murdoch Mysteries (2008)
flix add tmdb season 12786 0                     # Murdoch Mysteries Specials
flix add tmdb show 65930                         # My Hero Academia (2016)
flix add tmdb show 2288                          # Prison Break (2005)
flix add tmdb show 95396                         # Severance (2022)
flix add tmdb show 60573                         # Silicon Valley (2014)
flix add tmdb show 37680                         # Suits (2011)
flix add tmdb show 45782                         # Sword Art Online (2012)
flix add tmdb show 63404                         # Taskmaster (2015)
flix add tmdb season 63404 0                     # Taskmaster Specials
flix add tmdb show 48860                         # The Tomorrow People (2013)
flix add tmdb show 46331                         # Under the Dome (2013)
flix add tmdb show 1432                          # Veronica Mars (2004)
flix add tmdb show 186                           # Weeds (2005)
flix add tmdb show 63247                         # Westworld (2016)
flix add tmdb show 71912                         # The Witcher (2019)

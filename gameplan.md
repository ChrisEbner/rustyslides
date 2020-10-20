# As a User of this Binary I want to:
 - Send a Markdown message via http Request
 - Have it converted to a Slideshow
 - and returned a Link to download a Zipfile containing the Slideshow.

## Send a Markdown message via HTTP Request
 - Should work as message from a chatservice
 - Should work independent from a chatservice

## Have it converted to a slideshow
 - Should look good by default
 - Should work easily
 - No special syntax necessary
    - use Headlines for demarking new Slides,
    - use paragraphs as text



### Example Input:

```markdown

# Episode IV

It is a period of civil war.
Rebel spaceships, striking from a
hidden base, have won their
first victory against the evil
Galactic Empire. During the battle,
Rebel spies managed to steal
secret plans to the Empire's ultimate
weapon, the Death Star, an armoured
space station with enough power to
destroy an entire planet. Pursued by the
Empire's sinister agents, Princess
Leia races home aboard her starship,
custodian of the stolen plans that can save
her people and restore freedom to the galaxy...

```

### Todo: Add Example output

## Planned Features:

- Integration with Chat-Tool(s)
    - Mattermost // Slack
- Managing Slidedecks per User
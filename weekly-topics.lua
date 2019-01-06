local topics = {}

function topics.run()
  print('weekly topic identified')
  local topics = {
    ['August 26, 2018'] = "**The Weird Things Allosexuals Do**" .. newline ..
      "What things do allosexuals do that just confound you to no end?",
    ['September 02, 2018'] = "**Labels before there were labels**" .. newline ..
      "Before you discovered the term Demisexuality, did you go by a different label it descriptor? Did you try to fit into another label, or did you stand apart?",
    ['September 09, 2018'] = "**Man's Best Friends?**" .. newline ..
      "How is your relationship with animals when compared to people, easier, harder, about the same? Are animals something you can bond with other people over?",
    ['September 16, 2018'] = "**Freaky Friday**" .. newline ..
      "If you could live on the other side for one day/night, as an Allosexual, would you? What would you do?",
    ['September 23, 2018'] = "**Loud And Proud**" .. newline ..
      "Do you tell people you're Demisexual, do people in general know what it means?",
    ['September 30, 2018'] = "**Gossip Girl**" .. newline ..
      "How do you feel when friends talk about their relationships and encounters? Do you join in, sit back and listen, or try to change the topic of conversation?",
    ['October 07, 2018'] = "**Shades Of Gray**" .. newline ..
      "What do you see as the difference between Gray-Asexual and Demisexual, being as they are so closely related?" .. newline ..
      "http://wiki.asexuality.org/Gray-A/Grey-A",
    ['October 14, 2018'] = "**A Demisexual Mascot**" .. newline ..
      'Some in the Asexual community see Dragons as their "Mascot" of sorts. If Demisexuals had a "Mascot", what would it be?',
    ['October 21, 2018'] = "**Too Sexy For Their Shirts**" .. newline ..
      "Do you get crushes on people? What about squishes? How do you tell them apart and how do you deal with them?" .. newline ..
      "(Squish https://www.urbandictionary.com/define.php?term=squish)",
    ['October 28, 2018'] = "**Sex Sells, Or Does It?**" .. newline ..
      "So much of our society revolves around sexuality, especially in marketing. Do you find the sales technique of using sex to sell works for you, or is it a turn off?",
    ['November 04, 2018'] = "**Good Touch & Bad Touch**" .. newline ..
      "How do you feel about people touching you in platonic settings? Are you the touchy feely sort, or do you prefer to keep people at arms length? Is there a difference between those that are strangers, acquaintances and friends?",
    ['November 11, 2018'] = "**Greener On This Side**" .. newline ..
      "What is your favorite thing about being Demisexual? Do you find it easier to relax or focus, are some things more clear to you than to your Allosexual friends?",
    ['November 18, 2018'] = "**Tinder & Bumble & OK Cupid Oh My!**" .. newline ..
      "Dating Apps definitely weren't designed with Demisexual people in mind, even Allosexual people tend to hate them. Do you use them, do you tell people your demisexual on your profile, or do you find little of worth in them?",
    ['November 25, 2018'] = "**Separation Of Aesthetic & Sexual Attraction**" .. newline ..
      "The difference between recognizing someone as aesthetically pleasing and actually being sexually attracted to them or not is one of the main tenants of Demisexuality. But sometimes it's hard to discern this difference and other times it slaps you in the face making you think \"Yup, I'm definitely demisexual\". Have you ever had a feeling like this, perhaps this is what first brought you to the idea of being Demi? If so, tell us about it",
    ['December 02, 2018'] = "**Types & Preferences**" .. newline ..
      "Do you have a \"type\" that you prefer or are more innately attracted to? Maybe it's a personality type rather than a physical one, or maybe it's both. Are there certain qualities you look for in a romantic partner? What about qualities you wish to avoid?",
    ['December 09, 2018'] = "**The Birds & The Bees**" .. newline ..
      "What were your feelings when you were taught about sexuality? Did it come from your parents or from an educator? Was your reaction different than it was for others?",
    ['December 16, 2018'] = "**Game Of Sex**" .. newline ..
      "Sex scenes can be pretty prevalent in media, be it movies, television or books, etc. How do you feel when you come across these scenes, do you enjoy them, find them boring, skip over them or something else?",
    ['December 23, 2018'] = "**It's All Coming Up Aces**" .. newline ..
      "How do you feel about the larger Asexual community (of which Demisexuality is considered to be a subset of), do you have any thoughts about Asexuality in general?",
    ['December 30, 2018'] = "**Friendship Or Guilt Trip**" .. newline ..
      'Are you the sort to "catch some feels" for a longstanding friend. And if you do, do you tell them, or do you ignore it? Does it make everything a little brighter and interesting, or does it fill you with guilt and a sense of betrayal? How do you deal with these feelings?',
    -- these two Friendship questions were copied out of order.
    -- Can't undo the December 30th post so they're going to have to stay
    -- out of order.
    ['January 06, 2019'] = "**The Good Ship Friendship**" .. newline ..
      "Are the friendships you form different than those around you who aren't demisexual? Are they deeper, or more casual, slower to form or quick to bond? Do you form friendships easily or is it a delicate dance where people work their way closer to you?",
  }
  for week, topic in pairs(topics) do
    if containsIgnoreCase(title, week) then
      print('Posting Weekly topic')
      reply("" ..
        topic ..
        newline ..
        "*****" ..
        newline ..
        "Thanks to the Demisexuality discord for these topics https://discord.gg/4R8gKUa" ..
        footer
      )
    end
  end
end

return topics

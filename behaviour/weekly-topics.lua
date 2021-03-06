local topics = {}

-- Runs replies to the AutoModerator's weekly topic posts on /r/demisexuality
function topics.run()
  -- only do weekly topics in response to moderator bot posts
  -- so normal users cannot confuse this bot by posting a mock
  -- weekly discussion
  local modPost = (post and title) and author == 'AutoModerator'
  if not modPost then
    return
  end
  local discussionPost = containsIgnoreCase(
    title, 'Weekly /r/demisexuality discussion thread'
  )
  if not discussionPost then
    return
  end

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
    ['January 13, 2019'] = "**Friendship Or Guilt Trip**" .. newline ..
      'Are you the sort to "catch some feels" for a longstanding friend. And if you do, do you tell them, or do you ignore it? Does it make everything a little brighter and interesting, or does it fill you with guilt and a sense of betrayal? How do you deal with these feelings?',
    ['January 20, 2019'] = "**An Explanation Too Far**" .. newline ..
      "When talking about Demisexuality, do people ever say \"Oh isn't that how everyone is?\" or perhaps instead \"That's how all girls are right?\", maybe even \"I always want to get to know someone before I date them.\" What do you do when people misunderstand what your trying to say, that demisexuality is about not feeling sexual attraction up to a point, not wanting to take things slow etc… Do you try and explain the different types of attractions, or just go silent? ",
    ['January 27, 2019'] = "**Touch, I Remember Touch**" .. newline ..
      "Do you show platonic physical affection with your closest friends? (Not strangers/acquaintances) Do you hug, console, or caress, do you cuddle? If you do, what do you get from it and in what way is it different from a romantic and/or sexual relationship? If you don't, is that sort of a relationship (Often called a QPR (queer platonic relationship)) something you would want?",
    ['February 03, 2019'] = "** Missed Connection**" .. newline ..
      "Have you ever not been able to tell someone was flirting with someone else, or maybe even hitting on you... all while it was plain as day to your allosexual friends? Is it easier or harder to tell if they're focused on someone else?",
    ['February 10, 2019'] = "**Don't Read Too Much Into It**" .. newline ..
      "Has anyone ever mistakingly thought you were interested in them or even being flirtatious with them? Did you tell them you weren't? Did they believe you when you did?",
    ['February 17, 2019'] = "**Would You Look At That**" .. newline ..
      "Have family, friends, or romantic interests noticed how your different from themselves and/or others? Have they asked about it? How did you answer them or explain what Demisexual is?",
    ['February 24, 2019'] = "**The Masks That We Wear**" .. newline ..
      "Do you ever play along with friends and colleagues to get along.  Do you pretend to find others \"sexy\" when others comment about how overtly attractive someone is. Do you play along with people flirting with or touching you, even if you don't want to, so as to avoid embarrassment. How does it make you feel when you do this, do people ever see through the facade you've built?",
    ['March 03, 2019'] = "**A Romance By Any Other Name**" .. newline ..
      "Do you enjoy romance stories, be they written, plays, television or film? Do you prefer those that are long and drawn out or the more love at first sight style...?",
    ['March 10, 2019'] = "**No Sexuality Left Behind**" .. newline ..
      "What would you do to improve sex-ed in school? Would you talk about different sexualities, genders, and even experimentation? How far should sexual education in school go, is there anything that would be inappropriate to cover?",
    ['March 17, 2019'] = "**Last Stop, Everybody Out Of The Closet**" .. newline ..
      "What would you do to improve sex-ed in school? Would you talk about different sexualities, genders, and even experimentation? How far should sexual education in school go, is there anything that would be inappropriate to cover?",
    ['March 24, 2019'] = "**Poppycock, Balderdash & Twaddle**" .. newline ..
      "Some say that the word Demisexual is just made up, created by a teenager on a now defunct RP forum in 2006. Does that matter to you? Does it make your feelings any more or less real if it was?",
    ['March 31, 2019'] = "**Bonafide, Certified, Grade-A Demisexual**" .. newline ..
      "If there was a proven and infallible test you could take that could prove you either were or were not Demisexual, would you take it? What if the test said you weren't? Since there isn't such a test, would you like there to be?",
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

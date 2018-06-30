local newline = "\n\n"

local footer = newline .. "*****" .. newline ..
  "I'm a bot and this message was performed automatically. " ..
  "Contact /u/skeletonxf for issues. I'm also " ..
  "[open source](https://github.com/Skeletonxf/reddit-configurable-reply-bot)"

if comment or post then
  askingIfDemi = false
  for _, str in pairs({
    "am I demisexual",
    "am I demi?",
    "if I'm demisexual",
    "if I am demisexual",
    "make me demi?",
    "can you be demi and ",
    "I think I'm demi",
    "I think I am demi",
    "demi but I'm not sure",
    "demi but I am not sure",
    "demisexual but I'm not sure",
    "demisexual but I am not sure",
    "I might be demi",
    "I may be demi"
  }) do
    if post then
      if containsIgnoreCase(post, str) then
        askingIfDemi = true
      end
    end
    if title then
      if containsIgnoreCase(title, str) then
        askingIfDemi = true
      end
    end
  end

  if askingIfDemi then
    print('Found someone asking if they are demi')
    reply(
      "It looks like you might be asking if you're Demisexual. " ..
      "If so, you've come to the right place!" ..
      newline ..
      "We have a pinned [Links and Resources Masterpost](https://www.reddit.com/r/demisexuality/comments/8pn8pt/am_i_demi_links_and_resources_masterpost/) " ..
      "with lots of information which may be helpful to you, such as " ..
      "[Could I be Demisexual](https://web.archive.org/web/20170210212104/http://demisexuality.org/articles/could-i-be-demisexual/) " ..
      "and [Am I Demisexual if](https://web.archive.org/web/20170210210629/http://demisexuality.org/articles/am-i-demisexual-if/)." ..
      newline ..
      "I'm a bot so I don't know what you actually said. " ..
      "Maybe you're just looking for reassurance " ..
      "in which case remember you know yourself best." ..
      footer
    )
  end
end

local message = ""

if title and (not post) then
  -- link post
  -- check for http://demisexuality.freeforums.net/
  -- https://www.asexuality.org/.*?/forum/.*?/
  --
  -- https://regex101.com/
  -- https://docs.rs/regex/0.2.10/regex/#syntax
  local link = toLowercase(link)
  if matchesRegex(link, "reddit.com/r/.*?/comments/") and
        (not contains(link, "reddit.com/r/demisexuality/")) then
      print('found regex match', title, link)
  end
  return
end

local words = {
  ace = {
    words = {"Asexuality", "Asexual"},
    reply = (
      "An asexual is a person who does not experience sexual attraction. " ..
      "[Learn More](http://www.asexuality.org/home/?q=overview.html)" ..
      newline
    ),
  },
  demi = {
    words = {"Demisexual", "Demisexuality"},
    reply = (
      "A demisexual is a person who may experience sexual attraction " ..
      "but only after forming a strong emotion connection with " ..
      "that person(s). " ..
      "[Learn More](https://www.reddit.com/r/demisexuality/" ..
      "comments/7v2iwn/links_and_resources_masterpost/)" ..
      newline
    ),
  },
  grey = {
    words = {"Gray-A", "Grey-A", "Gray-Asexuality", "Grey-Asexuality",
      "Graysexual", "Greysexual"
    },
    reply = (
      "A grey asexual is a person at neither end of the spectrum on " ..
      "(a)sexual attraction. It can be used as an umbrella term for those " ..
      "who do not feel they fit as allosexual or asexual. " ..
      "[Learn More](http://www.asexuality.org/" ..
      "wiki/index.php?title=Gray-A_/_Grey-A)" ..
      newline
    ),
  },
  auto = {
    words = {"Autochorisexuality", "Autochoris", "Autochorisexual",
      "Autochorissexual", "Autochorissexuality"
    },
    reply = (
      "An autochorisexual person is in a subset of asexuality " ..
      "where there is a disconnect between oneself and a sexual target " ..
      "or object. For example a lack of desire to be a participant in " ..
      "sexual activies though still fantastising about sex. " ..
      "[Learn More](http://asexuals.wikia.com/wiki/Autochorissexual)" ..
      newline
    ),
  },
  bi = {
    words = {"Bisexuality", "Bisexual"},
    reply = (
      "A bisexual person is a person who experiences sexual attraction " ..
      "to at least two genders. Some may be only attracted to men/women " ..
      "while others may consider themselves attracted to same sex and " ..
      "different sex - not excluding minority genders. " ..
      "[~~Learn More~~](https://www.reddit.com/r/SexualityDefBot/" ..
      "comments/510m0s/definitions/)" ..
      newline
    ),
  },
  pan = {
    words = {"Pansexuality", "Pansexual"},
    reply = (
      "A pansexual person is a person who experiences sexual attraction " ..
      "irrespective of gender / to all genders. Some may consider themseles " ..
      "to be gender blind as apposed to gender attracted or to be attracted " ..
      "to a person's personality rather than gender or appearance. " ..
      "[~~Learn More~~](https://www.reddit.com/r/SexualityDefBot/" ..
      "comments/510m0s/definitions/)" ..
      newline
    ),
  }
}

-- the order is undefined due to using pairs
-- ideally the order should be the order of mentioning in the comment
-- this will do for now
for _, word in pairs(words) do
  local found = false
  for _, word in ipairs(word.words) do
    if containsIgnoreCase(comment or post, "[[" .. word .. "]]") then
      found = true
    end
  end
  if found then
    message = message .. word.reply
  end
end

if message ~= "" then
  reply(message .. footer)
end

--error 'crash the bot'
--return true

import praw
from config_bot import * # get data
r = praw.Reddit(USER_AGENT)
r.set_oauth_app_info(REDDIT_ID,REDDIT_SECRET,REDDIT_URI)
#print(r.get_access_information(APP_CODE))
print(r.refresh_access_information(REFRESH_TOKEN))
#print(r.get_authorize_url('...',SCOPE,True))
# tutorial https://www.reddit.com/r/GoldTesting/comments/3cm1p8/how_to_make_your_bot_use_oauth2/
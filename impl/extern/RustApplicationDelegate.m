#import "RustObject.h"
#import <UIKit/UIKit.h>

typedef struct RustObjectPtr RustApplicationDelegatePtr;

BOOL RustApplicationDelegateCreate(RustApplicationDelegatePtr *out);
void RustApplicationDelegateDestroy(RustApplicationDelegatePtr obj);
UIViewController *RustApplicationDelegateCreateRootViewController(RustApplicationDelegatePtr obj) NS_RETURNS_RETAINED;
BOOL RustApplicationDelegateDidFinishLaunching(RustApplicationDelegatePtr obj);

@interface RustApplicationDelegate : UIResponder <UIApplicationDelegate>
@end

@implementation RustApplicationDelegate {
  RustApplicationDelegatePtr _obj;
  UIWindow *_window;
}

- (instancetype)init {
  self = [super init];
  if (self) {
    BOOL success = RustApplicationDelegateCreate(&_obj);
    if (!success) {
      return nil;
    }
  }
  return self;
}

- (void)dealloc {
  RustApplicationDelegateDestroy(_obj);
  [_window release];
  [super dealloc];
}

- (UIWindow *)window {
  return _window;
}

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
  _window = [[UIWindow alloc] initWithFrame:[UIScreen mainScreen].bounds];
  UIViewController *rootViewController = RustApplicationDelegateCreateRootViewController(_obj);
  _window.rootViewController = rootViewController;
  [rootViewController release];
  [_window makeKeyAndVisible];

  return RustApplicationDelegateDidFinishLaunching(_obj);
}

@end

Class RustApplicationDelegateClass() {
  return [RustApplicationDelegate class];
}

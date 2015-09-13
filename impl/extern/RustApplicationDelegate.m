#import <UIKit/UIKit.h>

struct RustObjectPtr {
    void *data;
    void *vtable;
};

typedef struct RustObjectPtr RustApplicationDelegatePtr;

BOOL RustApplicationDelegateCreate(RustApplicationDelegatePtr *out);
void RustApplicationDelegateDestroy(RustApplicationDelegatePtr obj);
BOOL RustApplicationDelegateDidFinishLaunching(RustApplicationDelegatePtr obj);

@interface RustApplicationDelegate : UIResponder <UIApplicationDelegate>
@end

@implementation RustApplicationDelegate {
    RustApplicationDelegatePtr _obj;
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
  [super dealloc];
}

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
  return RustApplicationDelegateDidFinishLaunching(_obj);
}

@end

Class RustApplicationDelegateClass() {
  return [RustApplicationDelegate class];
}

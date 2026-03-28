# AuthPicker

### AuthPicker is a SSH authentication agent proxy meant to separate which keys are visible to which hosts. This solves the problem of having too many keys added to your agent, which when presented to a remote sshd, reaches the MaxAuthTries and fails authenticating.


### Instead of being forced to change the sshd configuration on every host you use, you can limit what they at different points with AuthPicker.

### AuthPicker works on a per connection or per device bases. 
### Per connection example:
> Say you want to only expose one key when connecting to a remote session, yet allow all of them to be available once connected for features dependent on **ForwardAgent** to work.
> You can set AuthPicker to only allow key based on the comment, and after the key is given once, it will then expose every key.

### Per device example:
> Only keys with comments tags with the current device will be exposed to this device and no other, this can work with authenticator forwarding for remote hosts ( if you are using a jump host ).

// HomeScreen.js
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
// import {Header} from './Header';
// import {heading} from './Typography'

const HomeScreen = () => {
  return (
    <View style={styles.container}>
      <Text style={styles.text_01}>
        Home Screen
      </Text>
    </View>
  );
};

// const HomeScreen = () => {
//   return (
//     <View>
//     <Header title="Welcome to React Native"/>
//     <Text style={heading}>Step One</Text>
//     <Text>
//       Edit App.js to change this screen and turn it
//       into your app.
//     </Text>
//     <Text style={heading}>See Your Changes</Text>
//     <Text>
//       Press Cmd + R inside the simulator to reload
//       your app’s code.
//     </Text>
//     <Text style={heading}>Debug</Text>
//     <Text>
//       Press Cmd + M or Shake your device to open the
//       React Native Debug Menu.
//     </Text>
//     <Text style={heading}>Learn</Text>
//     <Text>
//       Read the docs to discover what to do next:
//     </Text>
//    </View>  
//   );
// };
  

export default HomeScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#999',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: 16,
    textAlign: 'center',    
  },
  text_01: {
    flex: 1,
    // backgroundColor: '#1111',
    // alignItems: 'center',
    // justifyContent: 'center',
    fontSize: 53,
  },

});


import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const ChangeNowScreen = () => {
  return (
    <View style={styles.container}>
      <Text>ChangeNow Screen</Text>
      <Text>
      Connect to third-party services (such as: ChangeNow) to facilitate users to quickly redeem vara
      </Text>
    </View>
  );
};

export default ChangeNowScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
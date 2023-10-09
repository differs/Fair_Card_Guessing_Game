import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const LotteryScreen = () => {
  return (
    <View style={styles.container}>
      <Text>Lottery Screen</Text>
    </View>
  );
};

export default LotteryScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
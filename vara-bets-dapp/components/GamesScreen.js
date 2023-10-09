import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const GamesScreen = () => {
  return (
    <View style={styles.container}>
      <Text>Games Screen</Text>
      <Text>
      A page: used to announce upcoming new games.
      </Text>
    </View>
  );
};

export default GamesScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});